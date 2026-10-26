use crate::core::disaster::{
    DisasterLevel, DisasterManager, DisasterMetrics, DisasterPolicy, TransportType,
};
use crate::core::transport_selector::{
    TransportMetrics, TransportSelector, TransportSelectorConfig,
};
use tracing::{info, warn};

pub struct AutoTransportSwitcher {
    disaster_manager: DisasterManager,
    policy: DisasterPolicy,
    current_level: DisasterLevel,
    current_transport: TransportType,
    selector: TransportSelector,
}

impl AutoTransportSwitcher {
    pub fn new(disaster_manager: DisasterManager, policy: DisasterPolicy) -> Self {
        Self {
            disaster_manager,
            policy,
            current_level: DisasterLevel::Normal,
            current_transport: TransportType::Mesh,
            selector: TransportSelector::new(TransportSelectorConfig::default()),
        }
    }

    pub fn update_and_evaluate(
        &mut self,
        metrics: &DisasterMetrics,
        eligible: &[TransportType],
    ) -> Option<TransportType> {
        let new_level = self.disaster_manager.evaluate(metrics);

        if new_level != self.current_level {
            info!(
                "DISASTER LEVEL CHANGE: {:?} -> {:?}",
                self.current_level, new_level
            );
            self.current_level = new_level;

            if let Some(recommended) = self.policy.get_recommended_transport(new_level) {
                if recommended != self.current_transport && eligible.contains(&recommended) {
                    warn!("NETWORK ADAPTATION: Switching to {:?}", recommended);
                    self.current_transport = recommended;
                    return Some(recommended);
                }
            }
        }

        let transport_metrics: TransportMetrics = metrics.into();
        if eligible.is_empty() {
            return None;
        }

        let available = self.available_transports(eligible);
        let selected = self.selector.select(&transport_metrics, &available);

        if selected != self.current_transport {
            info!(
                "ADAPTIVE SELECTION: {:?} -> {:?} (score-based)",
                self.current_transport, selected
            );
            self.current_transport = selected;
            return Some(selected);
        }

        None
    }

    fn available_transports(&self, eligible: &[TransportType]) -> Vec<TransportType> {
        let candidates = match self.current_level {
            DisasterLevel::Normal => vec![
                TransportType::Mesh,
                TransportType::WiFi,
                TransportType::Reticulum,
            ],
            DisasterLevel::Congested => vec![
                TransportType::Mesh,
                TransportType::WiFi,
                TransportType::Reticulum,
            ],
            DisasterLevel::Isolation => vec![
                TransportType::WiFi,
                TransportType::Bluetooth,
                TransportType::Mesh,
                TransportType::Reticulum,
            ],
            DisasterLevel::Blackout => vec![
                TransportType::Bluetooth,
                TransportType::LoRa,
                TransportType::Reticulum,
            ],
        };

        candidates
            .into_iter()
            .filter(|candidate| eligible.contains(candidate))
            .collect()
    }

    pub fn current_level(&self) -> DisasterLevel {
        self.current_level
    }

    pub fn current_transport(&self) -> TransportType {
        self.current_transport
    }

    /// Restore the selector state when Core cannot apply a proposed transport.
    /// Selection is only successful once the registry/active handle is updated.
    pub fn restore_transport(&mut self, transport: TransportType) {
        self.selector.force_transport(transport);
        self.current_transport = transport;
    }

    pub fn force_transport(&mut self, transport: TransportType) {
        self.selector.force_transport(transport);
        self.current_transport = transport;
    }

    pub fn selector(&self) -> &TransportSelector {
        &self.selector
    }
}
