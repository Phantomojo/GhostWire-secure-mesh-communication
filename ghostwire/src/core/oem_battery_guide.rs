//! oem_battery_guide.rs — Per-OEM battery optimization guidance
//!
//! Android OEMs implement non-standard battery killers that terminate
//! background processes more aggressively than stock Android.
//! This module detects the OEM and provides exact user-facing instructions
//! to whitelist Ghostwire from battery optimization.
//!
//! Data sourced from dontkillmyapp.com (the authoritative OEM battery
//! restriction database) and verified against current firmware versions.
//!
//! Used by the Android onboarding flow to show device-specific instructions
//! on the "Allow background operation" step.

use serde::{Deserialize, Serialize};

/// A single setup step shown to the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupStep {
    pub instruction: String,
    pub setting_path: String, // e.g. "Settings → Battery → Background usage limits"
    pub image_hint: Option<String>, // Screenshot filename hint for UI
}

/// OEM-specific battery exemption guide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OemBatteryGuide {
    pub oem_name: String,
    pub severity: BatterySeverity,
    pub steps: Vec<SetupStep>,
    pub notes: Option<String>,
    /// Android Settings intent action to open relevant settings directly
    /// (bypasses manual navigation where possible)
    pub deep_link_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatterySeverity {
    /// Standard Android behavior — no special steps needed
    Standard,
    /// Moderate restrictions — 1-2 steps needed
    Moderate,
    /// Aggressive restrictions — 3+ steps needed, known to kill mesh nodes
    Aggressive,
}

impl BatterySeverity {
    pub fn warning_text(&self) -> &'static str {
        match self {
            Self::Standard => "Standard Android — Ghostwire should work normally.",
            Self::Moderate => "Your device has mild background restrictions. Follow the steps below to keep Ghostwire running.",
            Self::Aggressive => "Your device aggressively kills background apps. These steps are required for Ghostwire to work reliably as a mesh node.",
        }
    }
}

/// Get the battery guide for the current device.
/// Manufacturer string should come from Android's `Build.MANUFACTURER`.
pub fn get_guide(manufacturer: &str, model: &str) -> OemBatteryGuide {
    let mfr = manufacturer.to_lowercase();
    let mdl = model.to_lowercase();

    if mfr.contains("samsung") {
        samsung_guide(&mdl)
    } else if mfr.contains("xiaomi") || mfr.contains("redmi") || mfr.contains("poco") {
        xiaomi_guide(&mdl)
    } else if mfr.contains("huawei") || mfr.contains("honor") {
        huawei_guide()
    } else if mfr.contains("oppo") || mfr.contains("realme") || mfr.contains("oneplus") {
        oppo_guide()
    } else if mfr.contains("vivo") || mfr.contains("iqoo") {
        vivo_guide()
    } else if mfr.contains("tecno") || mfr.contains("itel") || mfr.contains("infinix") {
        // Common in Kenya and East Africa
        transsion_guide()
    } else {
        stock_android_guide()
    }
}

fn samsung_guide(model: &str) -> OemBatteryGuide {
    // One UI 6+ promised standard behavior but still has Device Care
    let is_one_ui_6_plus = model.contains("s24") || model.contains("s23") ||
        model.contains("a54") || model.contains("a35") || model.contains("a25");

    if is_one_ui_6_plus {
        OemBatteryGuide {
            oem_name: "Samsung (One UI 6+)".into(),
            severity: BatterySeverity::Moderate,
            steps: vec![
                SetupStep {
                    instruction: "Open Settings and go to Battery".into(),
                    setting_path: "Settings → Battery".into(),
                    image_hint: Some("samsung_battery.png".into()),
                },
                SetupStep {
                    instruction: "Tap 'Background usage limits'".into(),
                    setting_path: "Settings → Battery → Background usage limits".into(),
                    image_hint: None,
                },
                SetupStep {
                    instruction: "Find Ghostwire and set it to 'Never sleeping'".into(),
                    setting_path: "...(find Ghostwire)... → Never sleeping".into(),
                    image_hint: Some("samsung_never_sleeping.png".into()),
                },
            ],
            notes: Some("One UI 6+ should preserve foreground services per Samsung's commitment. If Ghostwire still stops, apply the 'Never sleeping' setting.".into()),
            deep_link_action: Some("android.settings.BATTERY_SAVER_SETTINGS".into()),
        }
    } else {
        OemBatteryGuide {
            oem_name: "Samsung (One UI 5 and earlier)".into(),
            severity: BatterySeverity::Aggressive,
            steps: vec![
                SetupStep {
                    instruction: "Go to Settings → Apps → find Ghostwire".into(),
                    setting_path: "Settings → Apps → Ghostwire".into(),
                    image_hint: None,
                },
                SetupStep {
                    instruction: "Tap Battery → select 'Unrestricted'".into(),
                    setting_path: "...Ghostwire → Battery → Unrestricted".into(),
                    image_hint: Some("samsung_unrestricted.png".into()),
                },
                SetupStep {
                    instruction: "Go to Settings → Device Care → Battery → Background usage limits → remove Ghostwire from the list".into(),
                    setting_path: "Settings → Device Care → Battery → Background usage limits".into(),
                    image_hint: None,
                },
            ],
            notes: Some("Samsung Device Care may reset these settings after updates. Re-check if Ghostwire stops.".into()),
            deep_link_action: Some("android.settings.APPLICATION_DETAILS_SETTINGS".into()),
        }
    }
}

fn xiaomi_guide(_model: &str) -> OemBatteryGuide {
    OemBatteryGuide {
        oem_name: "Xiaomi / Redmi / POCO (MIUI)".into(),
        severity: BatterySeverity::Aggressive,
        steps: vec![
            SetupStep {
                instruction: "Go to Settings → Apps → Manage apps → find Ghostwire".into(),
                setting_path: "Settings → Apps → Manage apps → Ghostwire".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Tap 'Battery saver' and select 'No restrictions'".into(),
                setting_path: "...Ghostwire → Battery saver → No restrictions".into(),
                image_hint: Some("xiaomi_battery_saver.png".into()),
            },
            SetupStep {
                instruction: "Go back and enable 'Autostart'".into(),
                setting_path: "...Ghostwire → Autostart → On".into(),
                image_hint: Some("xiaomi_autostart.png".into()),
            },
            SetupStep {
                instruction: "In Security app → Permissions → Autostart → enable Ghostwire".into(),
                setting_path: "Security → Permissions → Autostart → Ghostwire".into(),
                image_hint: None,
            },
        ],
        notes: Some("MIUI is the most aggressive OEM battery killer. All 4 steps are required. May reset after MIUI updates.".into()),
        deep_link_action: Some("android.settings.APPLICATION_DETAILS_SETTINGS".into()),
    }
}

fn huawei_guide() -> OemBatteryGuide {
    OemBatteryGuide {
        oem_name: "Huawei / Honor (EMUI / HarmonyOS)".into(),
        severity: BatterySeverity::Aggressive,
        steps: vec![
            SetupStep {
                instruction: "Go to Settings → Battery → App launch".into(),
                setting_path: "Settings → Battery → App launch".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Find Ghostwire, disable 'Manage automatically', then enable 'Auto-launch', 'Secondary launch', and 'Run in background'".into(),
                setting_path: "App launch → Ghostwire → Manual → all three on".into(),
                image_hint: Some("huawei_app_launch.png".into()),
            },
            SetupStep {
                instruction: "Go to Settings → Apps → Apps → Ghostwire → Battery → enable 'Allow background activity'".into(),
                setting_path: "Apps → Ghostwire → Battery → Allow background activity".into(),
                image_hint: None,
            },
        ],
        notes: Some("Huawei's PowerGenie kills apps after 1 hour in background by default. These settings exempt Ghostwire specifically.".into()),
        deep_link_action: None,
    }
}

fn oppo_guide() -> OemBatteryGuide {
    OemBatteryGuide {
        oem_name: "OPPO / Realme / OnePlus (ColorOS / OxygenOS)".into(),
        severity: BatterySeverity::Moderate,
        steps: vec![
            SetupStep {
                instruction: "Go to Settings → Battery → Battery optimization".into(),
                setting_path: "Settings → Battery → Battery optimization".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Find Ghostwire and set to 'Don't optimize'".into(),
                setting_path: "Battery optimization → Ghostwire → Don't optimize".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Go to Settings → Apps → Ghostwire → Battery → Enable 'Allow background activity'".into(),
                setting_path: "Apps → Ghostwire → Battery → Allow background activity".into(),
                image_hint: None,
            },
        ],
        notes: None,
        deep_link_action: Some("android.settings.IGNORE_BATTERY_OPTIMIZATION_SETTINGS".into()),
    }
}

fn vivo_guide() -> OemBatteryGuide {
    OemBatteryGuide {
        oem_name: "Vivo / iQOO (FuntouchOS / OriginOS)".into(),
        severity: BatterySeverity::Aggressive,
        steps: vec![
            SetupStep {
                instruction: "Open iManager → App management → Autostart management → enable Ghostwire".into(),
                setting_path: "iManager → App management → Autostart → Ghostwire".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Go to Settings → Battery → Background app management → Ghostwire → Don't restrict".into(),
                setting_path: "Settings → Battery → Background apps → Ghostwire → Don't restrict".into(),
                image_hint: None,
            },
        ],
        notes: None,
        deep_link_action: None,
    }
}

fn transsion_guide() -> OemBatteryGuide {
    // Transsion brands (Tecno, Itel, Infinix) — dominant in Kenya/East Africa
    OemBatteryGuide {
        oem_name: "Tecno / Itel / Infinix (HiOS / XOS)".into(),
        severity: BatterySeverity::Moderate,
        steps: vec![
            SetupStep {
                instruction: "Go to Settings → Apps → find Ghostwire".into(),
                setting_path: "Settings → Apps → Ghostwire".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Tap Battery → select 'Unrestricted' or 'No restriction'".into(),
                setting_path: "Ghostwire → Battery → Unrestricted".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Go to Security Center → Power saver → Whitelist → add Ghostwire".into(),
                setting_path: "Security Center → Power saver → Whitelist → Ghostwire".into(),
                image_hint: None,
            },
        ],
        notes: Some("Transsion devices are common in East Africa. HiOS/XOS vary by firmware version — these steps cover most configurations.".into()),
        deep_link_action: Some("android.settings.APPLICATION_DETAILS_SETTINGS".into()),
    }
}

fn stock_android_guide() -> OemBatteryGuide {
    OemBatteryGuide {
        oem_name: "Stock Android (Pixel and others)".into(),
        severity: BatterySeverity::Standard,
        steps: vec![
            SetupStep {
                instruction: "Go to Settings → Apps → Ghostwire → Battery".into(),
                setting_path: "Settings → Apps → Ghostwire → Battery".into(),
                image_hint: None,
            },
            SetupStep {
                instruction: "Select 'Unrestricted' to allow full background activity".into(),
                setting_path: "Battery → Unrestricted".into(),
                image_hint: None,
            },
        ],
        notes: Some("Stock Android is the most permissive. Ghostwire's foreground service should run normally with just this setting.".into()),
        deep_link_action: Some("android.settings.IGNORE_BATTERY_OPTIMIZATION_SETTINGS".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_oems_return_valid_guides() {
        let oems = [
            ("samsung", "galaxy s24"),
            ("samsung", "galaxy a15"),
            ("xiaomi", "redmi note 13"),
            ("huawei", "p40 lite"),
            ("oppo", "a78"),
            ("vivo", "y36"),
            ("tecno", "camon 20"),
            ("google", "pixel 8"),
        ];
        for (mfr, model) in oems {
            let guide = get_guide(mfr, model);
            assert!(!guide.steps.is_empty(), "Guide for {} should have steps", mfr);
            assert!(!guide.oem_name.is_empty());
        }
    }

    #[test]
    fn transsion_detected_correctly() {
        // Tecno, Itel, Infinix are Transsion brands — dominant in Kenya
        let guide = get_guide("TECNO", "CAMON 20 Pro");
        assert!(guide.oem_name.contains("Tecno"));
        assert_eq!(guide.severity, BatterySeverity::Moderate);
    }
}
