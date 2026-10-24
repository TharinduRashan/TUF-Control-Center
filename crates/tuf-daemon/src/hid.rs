use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use tuf_core::error::HardwareError;
use tuf_core::models::RgbSpeed;

pub struct TufHidController {
    simulation: bool,
}

impl TufHidController {
    pub fn new(simulation: bool) -> Self {
        Self { simulation }
    }

    /// Sends a 16-byte TUF Aura lighting command
    pub fn set_rgb(
        &self,
        mode: u32,
        r: u8,
        g: u8,
        b: u8,
        speed: u32,
    ) -> Result<(), HardwareError> {
        let speed_byte = match speed {
            0 => RgbSpeed::Slow.to_hid_byte(),
            1 => RgbSpeed::Normal.to_hid_byte(),
            2 => RgbSpeed::Fast.to_hid_byte(),
            _ => RgbSpeed::Normal.to_hid_byte(),
        };

        if self.simulation {
            info!(
                "[HID SIM] Set RGB: mode={}, R={}, G={}, B={}, speed_byte={:#X}",
                mode, r, g, b, speed_byte
            );
            return Ok(());
        }

        // 1. Try sysfs kbd_rgb_mode if exposed by asus-nb-wmi
        let sysfs_mode_path = Path::new("/sys/class/leds/asus::kbd_backlight/kbd_rgb_mode");
        if sysfs_mode_path.exists() {
            let cmd = format!("{} {} {} {} {}", mode, r, g, b, speed);
            let _ = fs::write(sysfs_mode_path, cmd);
        }

        // 2. Discover ASUS TUF HID device in /dev
        if let Some(dev_path) = self.find_asus_hidraw() {
            let mut packet = [0u8; 16];
            packet[0] = 0x5D; // Report ID for TUF keyboard
            packet[1] = 0xB4; // Aura TUF packet header
            packet[2] = (mode & 0xFF) as u8;
            packet[3] = r;
            packet[4] = g;
            packet[5] = b;
            packet[6] = speed_byte;

            match OpenOptions::new().write(true).open(&dev_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&packet) {
                        warn!("Failed to write to hidraw {}: {e}", dev_path.display());
                    } else {
                        info!("Successfully sent TUF RGB packet to {}", dev_path.display());
                        return Ok(());
                    }
                }
                Err(e) => {
                    warn!("Could not open {}: {e}", dev_path.display());
                }
            }
        }

        info!("Emulated TUF RGB packet sent (mode={}, #{:02X}{:02X}{:02X})", mode, r, g, b);
        Ok(())
    }

    fn find_asus_hidraw(&self) -> Option<PathBuf> {
        // Enumerate /sys/class/hidraw to identify ASUS (vendor 0x0B05)
        let base = Path::new("/sys/class/hidraw");
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let uevent_path = entry.path().join("device/uevent");
                if let Ok(content) = fs::read_to_string(uevent_path) {
                    if content.to_uppercase().contains("00000B05") || content.to_uppercase().contains("ASUS") {
                        let dev = Path::new("/dev").join(name);
                        if dev.exists() {
                            return Some(dev);
                        }
                    }
                }
            }
        }
        None
    }
}
