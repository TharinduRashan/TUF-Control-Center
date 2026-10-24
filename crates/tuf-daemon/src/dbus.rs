use std::sync::Arc;
use tracing::info;
use zbus::interface;
use crate::hid::TufHidController;
use crate::polkit::PolkitChecker;
use crate::sysfs::SysfsController;

pub struct ControlCenterDaemon {
    sysfs: Arc<SysfsController>,
    hid: Arc<TufHidController>,
    polkit: Arc<PolkitChecker>,
    simulation: bool,
}

impl ControlCenterDaemon {
    pub fn new(
        sysfs: Arc<SysfsController>,
        hid: Arc<TufHidController>,
        polkit: Arc<PolkitChecker>,
        simulation: bool,
    ) -> Self {
        Self {
            sysfs,
            hid,
            polkit,
            simulation,
        }
    }
}

#[interface(name = "org.tuf.ControlCenter")]
impl ControlCenterDaemon {
    async fn ping(&self) -> zbus::fdo::Result<String> {
        Ok(format!(
            "TUF Control Center Daemon v{} (Simulation={})",
            tuf_core::version(),
            self.simulation
        ))
    }

    async fn set_throttle_thermal_policy(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        policy: u32,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        self.sysfs
            .set_throttle_thermal_policy(policy)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn get_throttle_thermal_policy(&self) -> zbus::fdo::Result<u32> {
        self.sysfs
            .get_throttle_thermal_policy()
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))
    }

    async fn set_platform_profile(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        profile: String,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        self.sysfs
            .set_platform_profile(&profile)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn get_platform_profile(&self) -> zbus::fdo::Result<String> {
        self.sysfs
            .get_platform_profile()
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))
    }

    async fn set_battery_limit(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        limit_pct: u32,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        self.sysfs
            .set_battery_limit(limit_pct)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn get_battery_limit(&self) -> zbus::fdo::Result<u32> {
        self.sysfs
            .get_battery_limit()
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))
    }

    async fn set_cpu_epp(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        epp: String,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        self.sysfs
            .set_cpu_epp(&epp)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn set_keyboard_brightness(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        brightness: u32,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        self.sysfs
            .set_keyboard_brightness(brightness)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn set_keyboard_rgb(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        mode: u32,
        r: u8,
        g: u8,
        b: u8,
        speed: u32,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        self.hid
            .set_rgb(mode, r, g, b, speed)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn set_gpu_mode(
        &self,
        #[zbus(header)] _hdr: zbus::message::Header<'_>,
        mode: String,
    ) -> zbus::fdo::Result<()> {
        let _ = self.polkit.check_authorization(
            0,
            0,
            "org.tuf.controlcenter.set-hardware-settings",
        ).map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;

        info!("Daemon processing GPU Mode switch request: {}", mode);
        // Integrate with supergfxctl or runtime power-management
        Ok(())
    }

    async fn get_gpu_mode(&self) -> zbus::fdo::Result<String> {
        Ok("hybrid".into())
    }
}
