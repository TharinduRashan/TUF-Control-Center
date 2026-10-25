use crate::hid::TufHidController;
use crate::polkit::PolkitChecker;
use crate::sysfs::SysfsController;
use std::sync::Arc;
use tracing::info;
use zbus::interface;

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
                let uid = dbus_proxy
                    .get_connection_unix_user(bus_name)
                    .await
                    .unwrap_or(1000);
                (sender.as_str().to_string(), uid)
            } else {
                (sender.as_str().to_string(), 1000)
            }
        } else {
            (String::new(), 1000)
        };

        self.polkit
            .check_authorization(&sender_name, uid, action_id)
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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

        self.sysfs
            .set_keyboard_brightness(brightness)
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

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
        self.check_auth(conn, &hdr, "org.tuf.controlcenter.set-hardware-settings")
            .await?;

        let mode_lower = mode.trim().to_lowercase();
        info!("Daemon processing GPU Mode switch request: {}", mode_lower);

        match mode_lower.as_str() {
            "dedicated" => {
                let has_mux = std::path::Path::new("/sys/devices/platform/asus-nb-wmi/gpu_mux")
                    .exists()
                    || std::path::Path::new("/sys/bus/platform/devices/asus-nb-wmi/dgpu_disable")
                        .exists();
                if !has_mux {
                    return Err(zbus::fdo::Error::NotSupported(
                        "Hardware MUX switch is not present on this model (FA506NC). Dedicated GPU mode is unsupported.".into(),
                    ));
                }

                if self.simulation {
                    info!("[GPU SIM] Emulated switch to Dedicated GPU mode");
                    return Ok(());
                }

                let output = std::process::Command::new("supergfxctl")
                    .args(["-m", "Dedicated"])
                    .output();
                match output {
                    Ok(out) if out.status.success() => Ok(()),
                    Ok(out) => Err(zbus::fdo::Error::Failed(format!(
                        "supergfxctl failed: {}",
                        String::from_utf8_lossy(&out.stderr)
                    ))),
                    Err(e) => Err(zbus::fdo::Error::Failed(format!(
                        "supergfxctl invocation error: {e}"
                    ))),
                }
            }
            "hybrid" | "integrated" => {
                if self.simulation {
                    info!("[GPU SIM] Emulated switch to GPU mode: {}", mode_lower);
                    return Ok(());
                }

                let target_mode = if mode_lower == "hybrid" {
                    "Hybrid"
                } else {
                    "Integrated"
                };
                let output = std::process::Command::new("supergfxctl")
                    .args(["-m", target_mode])
                    .output();

                match output {
                    Ok(out) if out.status.success() => {
                        info!("supergfxctl successfully applied GPU mode: {}", target_mode);
                        Ok(())
                    }
                    Ok(out) => {
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        Err(zbus::fdo::Error::Failed(format!(
                            "supergfxctl returned error: {stderr}"
                        )))
                    }
                    Err(_) => {
                        let pci_power_control =
                            std::path::Path::new("/sys/bus/pci/devices/0000:01:00.0/power/control");
                        if pci_power_control.exists() {
                            let val = if mode_lower == "integrated" {
                                "auto"
                            } else {
                                "on"
                            };
                            std::fs::write(pci_power_control, val).map_err(|e| {
                                zbus::fdo::Error::Failed(format!(
                                    "Failed to write PCI power control: {e}"
                                ))
                            })?;
                            info!("Applied PCI power control = {} for dGPU", val);
                            Ok(())
                        } else {
                            Err(zbus::fdo::Error::Failed(
                                "GPU mode switching requires 'supergfxctl'. Please install supergfxctl: sudo apt install supergfxctl".into(),
                            ))
                        }
                    }
                }
            }
            _ => Err(zbus::fdo::Error::InvalidArgs(format!(
                "Invalid GPU mode '{mode}'. Valid modes are 'hybrid' or 'integrated'."
            ))),
        }
    }

    async fn get_gpu_mode(&self) -> zbus::fdo::Result<String> {
        if self.simulation {
            return Ok("hybrid".into());
        }

        if let Ok(out) = std::process::Command::new("supergfxctl").arg("-g").output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
                if !s.is_empty() {
                    return Ok(s);
                }
            }
        }

        let runtime_status =
            std::path::Path::new("/sys/bus/pci/devices/0000:01:00.0/power/runtime_status");
        if let Ok(s) = std::fs::read_to_string(runtime_status) {
            if s.trim() == "suspended" {
                return Ok("integrated".into());
            }
        }

        Ok("hybrid".into())
    }
}
