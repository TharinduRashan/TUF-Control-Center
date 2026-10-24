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
        caller_pid: u32,
        caller_uid: u32,
        action_id: &str,
    ) -> Result<(), HardwareError> {
        if !self.enabled {
            return Ok(());
        }

        // If caller is root (uid 0), always permit
        if caller_uid == 0 {
            return Ok(());
        }

        // Query pkcheck
        let output = Command::new("pkcheck")
            .arg("--action-id")
            .arg(action_id)
            .arg("--process")
            .arg(caller_pid.to_string())
            .arg("--allow-user-interaction")
            .output();

        match output {
            Ok(res) if res.status.success() => {
                info!("Polkit authorization granted for action {action_id} (PID {caller_pid})");
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
                // If pkcheck is not installed or running in container, fallback gracefully
                warn!("pkcheck not available ({e}), allowing operation for local user {caller_uid}");
                Ok(())
            }
        }
    }
}
