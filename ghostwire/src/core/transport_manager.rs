use crate::core::disaster::{
    DisasterLevel, DisasterManager, DisasterMetrics, DisasterPolicy, TransportType,
};
use tracing::{info, warn};

/// Decides which transport to use based on network conditions and policy.
pub struct AutoTransportSwitcher {
    disaster_manager: DisasterManager,
    policy: DisasterPolicy,
    current_level: DisasterLevel,
    current_transport: TransportType,
}

impl AutoTransportSwitcher {
    pub fn new(disaster_manager: DisasterManager, policy: DisasterPolicy) -> Self {
        Self {
            disaster_manager,
            policy,
            current_level: DisasterLevel::Normal,
            current_transport: TransportType::StealthTcp, // Default start
        }
    }

    /// Evaluates metrics and returns the recommended transport if a change is needed.
    pub fn update_and_evaluate(&mut self, metrics: &DisasterMetrics) -> Option<TransportType> {
        let new_level = self.disaster_manager.evaluate(metrics);

        if new_level != self.current_level {
            info!(
                "DISASTER LEVEL CHANGE: {:?} -> {:?}",
                self.current_level, new_level
            );
            self.current_level = new_level;

            if let Some(recommended) = self.policy.get_recommended_transport(new_level) {
                if recommended != self.current_transport {
                    warn!("NETWORK ADAPTATION: Switching to {:?}", recommended);
                    self.current_transport = recommended;
                    return Some(recommended);
                }
            }
        }

        None
    }

    pub fn current_level(&self) -> DisasterLevel {
        self.current_level
    }

    pub fn current_transport(&self) -> TransportType {
        self.current_transport
    }
}
