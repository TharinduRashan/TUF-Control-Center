use std::process::Command;
use tracing::{info, warn};
use tuf_core::error::HardwareError;

pub struct PolkitChecker {
    enabled: bool,
}

impl PolkitChecker {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn check_authorization(
        &self,
        sender_bus_name: &str,
        caller_uid: u32,
        action_id: &str,
    ) -> Result<(), HardwareError> {
        if !self.enabled {
            return Ok(());
        }

        if caller_uid == 0 {
            return Ok(());
        }

        let mut cmd = Command::new("pkcheck");
        cmd.arg("--action-id").arg(action_id);
        if !sender_bus_name.is_empty() {
            cmd.arg("--system-bus-name").arg(sender_bus_name);
        }
        cmd.arg("--allow-user-interaction");

        match cmd.output() {
            Ok(res) if res.status.success() => {
                info!("Polkit authorization granted for action {action_id} (D-Bus sender {sender_bus_name})");
                Ok(())
            }
            Ok(res) => {
                let stderr = String::from_utf8_lossy(&res.stderr);
                warn!("Polkit authorization denied for {action_id}: {stderr}");
                Err(HardwareError::PermissionDenied(format!(
                    "PolicyKit authorization failed for action {action_id}"
                )))
            }
            Err(e) => {
                warn!("pkcheck not available ({e}), allowing operation for local user {caller_uid}");
                Ok(())
            }
        }
    }
}
