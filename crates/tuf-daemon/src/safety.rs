use crate::sysfs::SysfsController;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::info;

pub struct SafetyWatchdog {
    running: Arc<AtomicBool>,
}

impl Default for SafetyWatchdog {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyWatchdog {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn setup_signal_handlers(&self, sysfs: Arc<SysfsController>) {
        let running = self.running.clone();
        tokio::spawn(async move {
            if let Ok(()) = tokio::signal::ctrl_c().await {
                info!("Termination signal received. Restoring safe firmware thermal defaults...");
                running.store(false, Ordering::SeqCst);
                let _ = sysfs.set_throttle_thermal_policy(0); // Balanced / firmware default
                let _ = sysfs.set_platform_profile("balanced");
                info!("Safe hardware state restored. Exiting cleanly.");
                std::process::exit(0);
            }
        });
    }
}
