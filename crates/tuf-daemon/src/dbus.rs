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

    async fn check_auth(
        &self,
        conn: &zbus::Connection,
        hdr: &zbus::message::Header<'_>,
        action_id: &str,
    ) -> zbus::fdo::Result<()> {
        let (sender_name, uid) = if let Some(sender) = hdr.sender() {
            if let Ok(dbus_proxy) = zbus::fdo::DBusProxy::new(conn).await {
                let bus_name: zbus::names::BusName<'_> = sender.clone().into();
                let uid = dbus_proxy.get_connection_unix_user(bus_name).await.unwrap_or(1000);
                (sender.as_str().to_string(), uid)
            } else {
                (sender.as_str().to_string(), 1000)
            }
        } else {
            (String::new(), 1000)
        };

        self.polkit.check_authorization(&sender_name, uid, action_id)
            .map_err(|e| zbus::fdo::Error::AuthFailed(e.to_string()))?;
        Ok(())
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
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        policy: u32,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

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
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        profile: String,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

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
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        limit_pct: u32,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

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
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        epp: String,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

        self.sysfs
            .set_cpu_epp(&epp)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn set_keyboard_brightness(
        &self,
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        brightness: u32,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

        self.sysfs
            .set_keyboard_brightness(brightness)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn set_keyboard_rgb(
        &self,
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        mode: u32,
        r: u8,
        g: u8,
        b: u8,
        speed: u32,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

        self.hid
            .set_rgb(mode, r, g, b, speed)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    async fn set_gpu_mode(
        &self,
        #[zbus(connection)] conn: &zbus::Connection,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        mode: String,
    ) -> zbus::fdo::Result<()> {
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings").await?;

        info!("Daemon processing GPU Mode switch request: {}", mode);
        Ok(())
    }

    async fn get_gpu_mode(&self) -> zbus::fdo::Result<String> {
        Ok("hybrid".into())
    }
}
