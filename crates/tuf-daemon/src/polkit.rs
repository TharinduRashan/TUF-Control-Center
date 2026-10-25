use std::process::Command;
use tracing::{error, info, warn};
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

        // Root caller (UID 0) is inherently authorized
        if caller_uid == 0 {
            return Ok(());
        }

        let mut cmd = Command::new("pkcheck");
        cmd.arg("--action-id").arg(action_id);
        if !sender_bus_name.is_empty() {
            cmd.arg("--system-bus-name").arg(sender_bus_name);
        } else {
            return Err(HardwareError::PermissionDenied(
                "Cannot authorize action: D-Bus sender bus name is missing".to_string(),
            ));
        }
        cmd.arg("--allow-user-interaction");

        match cmd.output() {
            Ok(res) if res.status.success() => {
                info!("Polkit authorization granted for action {action_id} (D-Bus sender {sender_bus_name})");
                Ok(())
            }
            Ok(res) => {
                let stderr = String::from_utf8_lossy(&res.stderr);
                warn!(
                    "Polkit authorization denied for {action_id} (caller {caller_uid}): {stderr}"
                );
                Err(HardwareError::PermissionDenied(format!(
                    "PolicyKit authorization failed for action {action_id}: {stderr}"
                )))
            }
            Err(e) => {
                // Fail-closed: Never allow unprivileged access when authorization checker cannot be executed
                error!("pkcheck execution failed: {e}. Denying privileged action for user {caller_uid}");
                Err(HardwareError::PermissionDenied(format!(
                    "PolicyKit authorization service unavailable: {e}"
                )))
            }
        }
    }
}
