//! Power management for GhostWire mesh nodes.
//!
//! Implements power modes and scheduling to balance privacy (cover traffic,
//! mixing delays) against battery life. Based on research from Briar
//! (foreground service + power management) and Bitchat (PowerManager).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

/// Power mode determines cover traffic rate, mixing delay, and scan frequency.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PowerMode {
    /// Maximum privacy: aggressive cover traffic, long mixing delays, frequent scans.
    /// Use when on AC power or user explicitly requests.
    Performance,
    /// Balanced: moderate cover traffic and mixing delays.
    #[default]
    Balanced,
    /// Battery saver: minimal cover traffic, short mixing delays, infrequent scans.
    /// Use when on battery or user requests power saving.
    PowerSaver,
    /// Ultra-low power: no cover traffic, minimal mixing, scan only when needed.
    /// Use for IoT or extreme battery conservation.
    UltraLow,
}

impl PowerMode {
    /// (cover_min_ms, cover_max_ms, mixing_delay_ms, scan_interval_secs, padding_aggressive)
    pub fn params(self) -> (u64, u64, u64, u64, bool) {
        match self {
            Self::Performance => (3_000, 8_000, 200, 10, true),
            Self::Balanced => (5_000, 12_000, 100, 30, true),
            Self::PowerSaver => (10_000, 30_000, 50, 60, false),
            Self::UltraLow => (0, 0, 0, 300, false),
        }
    }

    /// Whether cover traffic should be generated in this mode.
    pub fn cover_traffic_enabled(self) -> bool {
        !matches!(self, Self::UltraLow)
    }

    /// Whether mixing delays should be applied.
    pub fn mixing_enabled(self) -> bool {
        !matches!(self, Self::UltraLow)
    }

    /// Scan interval in seconds.
    pub fn scan_interval_secs(self) -> u64 {
        self.params().3
    }
}

/// Power manager tracks current mode and battery state.
pub struct PowerManager {
    mode: Arc<AtomicU8>,
    battery_level: Arc<AtomicU8>, // 0-100
    on_ac_power: Arc<std::sync::atomic::AtomicBool>,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            mode: Arc::new(AtomicU8::new(PowerMode::Balanced as u8)),
            battery_level: Arc::new(AtomicU8::new(100)),
            on_ac_power: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    pub fn current_mode(&self) -> PowerMode {
        match self.mode.load(Ordering::Relaxed) {
            0 => PowerMode::Performance,
            1 => PowerMode::Balanced,
            2 => PowerMode::PowerSaver,
            3 => PowerMode::UltraLow,
            _ => PowerMode::Balanced,
        }
    }

    pub fn set_mode(&self, mode: PowerMode) {
        self.mode.store(mode as u8, Ordering::Relaxed);
    }

    pub fn update_battery(&self, level: u8, on_ac: bool) {
        self.battery_level.store(level.min(100), Ordering::Relaxed);
        self.on_ac_power.store(on_ac, Ordering::Relaxed);

        // Auto-adjust mode based on battery
        if on_ac {
            self.set_mode(PowerMode::Performance);
        } else if level < 15 {
            self.set_mode(PowerMode::UltraLow);
        } else if level < 30 {
            self.set_mode(PowerMode::PowerSaver);
        } else {
            self.set_mode(PowerMode::Balanced);
        }
    }

    pub fn battery_level(&self) -> u8 {
        self.battery_level.load(Ordering::Relaxed)
    }

    /// Read battery level from Linux sysfs (/sys/class/power_supply/).
    /// Returns (level, on_ac_power). Returns None if no battery found.
    pub fn read_battery_sysfs(&self) -> Option<(u8, bool)> {
        // Try common battery paths
        for bat_name in &["BAT0", "BAT1", "battery"] {
            let cap_path = format!("/sys/class/power_supply/{}/capacity", bat_name);
            let status_path = format!("/sys/class/power_supply/{}/status", bat_name);
            if let Ok(cap_str) = std::fs::read_to_string(&cap_path) {
                if let Ok(level) = cap_str.trim().parse::<u8>() {
                    // Check if on AC power
                    let on_ac = std::fs::read_to_string("/sys/class/power_supply/AC/online")
                        .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/ACAD/online"))
                        .map(|s| s.trim() == "1")
                        .unwrap_or(false);
                    return Some((level, on_ac));
                }
            }
        }
        None
    }

    /// Update battery from sysfs (call periodically).
    pub fn refresh_battery(&self) {
        if let Some((level, on_ac)) = self.read_battery_sysfs() {
            self.update_battery(level, on_ac);
        }
    }

    pub fn on_ac_power(&self) -> bool {
        self.on_ac_power.load(Ordering::Relaxed)
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_mode_params() {
        let (cmin, cmax, mix, scan, pad) = PowerMode::Performance.params();
        assert_eq!(cmin, 3_000);
        assert!(pad);

        let (cmin, cmax, mix, scan, pad) = PowerMode::UltraLow.params();
        assert_eq!(cmin, 0);
        assert!(!pad);
    }

    #[test]
    fn test_auto_battery_adjust() {
        let pm = PowerManager::new();
        pm.update_battery(5, false);
        assert_eq!(pm.current_mode(), PowerMode::UltraLow);

        pm.update_battery(50, false);
        assert_eq!(pm.current_mode(), PowerMode::Balanced);

        pm.update_battery(80, true);
        assert_eq!(pm.current_mode(), PowerMode::Performance);
    }

    #[test]
    fn test_cover_traffic_enabled() {
        assert!(PowerMode::Performance.cover_traffic_enabled());
        assert!(PowerMode::Balanced.cover_traffic_enabled());
        assert!(PowerMode::PowerSaver.cover_traffic_enabled());
        assert!(!PowerMode::UltraLow.cover_traffic_enabled());
    }
}
