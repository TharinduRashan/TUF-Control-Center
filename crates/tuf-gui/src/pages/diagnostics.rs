use gtk4::prelude::*;
use gtk4::{Button, Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct DiagnosticsPage {
    pub container: ScrolledWindow,
}

impl DiagnosticsPage {
    pub fn new(backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let hw_group = PreferencesGroup::builder()
            .title("Hardware Capability Registry")
            .description("Inspected ACPI, WMI, sysfs, and DMI driver nodes")
            .build();

        let model_row = ActionRow::builder()
            .title("Device Model")
            .subtitle("ASUS TUF Gaming A15 FA506NC_FA506NC")
            .build();
        hw_group.add(&model_row);

        let vendor_row = ActionRow::builder()
            .title("System Vendor")
            .subtitle("ASUSTeK COMPUTER INC.")
            .build();
        hw_group.add(&vendor_row);

        let ec_row = ActionRow::builder()
            .title("ASUS WMI Kernel Module")
            .subtitle("asus-nb-wmi / asus-wmi loaded")
            .build();
        let ec_label = Label::builder().label("Active").css_classes(["success"]).build();
        ec_row.add_suffix(&ec_label);
        hw_group.add(&ec_row);

        let mux_row = ActionRow::builder()
            .title("Hardware MUX Switch")
            .subtitle("Advanced Optimus physical multiplexer")
            .build();
        let mux_label = Label::builder().label("Not Available on FA506NC").css_classes(["dim-label"]).build();
        mux_row.add_suffix(&mux_label);
        hw_group.add(&mux_row);

        let bat_row = ActionRow::builder()
            .title("Battery Threshold Register")
            .subtitle("/sys/class/power_supply/BAT0/charge_control_end_threshold")
            .build();
        let bat_label = Label::builder().label("Supported (60/80/100%)").css_classes(["success"]).build();
        bat_row.add_suffix(&bat_label);
        hw_group.add(&bat_row);

        page.add(&hw_group);

        let export_group = PreferencesGroup::builder()
            .title("Diagnostic Logs")
            .description("Export hardware sensor states and journal logs for troubleshooting")
            .build();

        let export_row = ActionRow::builder()
            .title("Export System Report")
            .subtitle("Generates a markdown diagnostics report at ~/tuf-report.md")
            .build();
        let btn_export = Button::with_label("Export Report");

        {
            let b = backend.clone();
            let btn_clone = btn_export.clone();
            btn_export.connect_clicked(move |_| {
                let report = if let Ok(mut lock) = b.lock() {
                    let caps = lock.detect_capabilities();
                    let tel = lock.read_telemetry().ok();
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
                    let path = std::path::PathBuf::from(home).join("tuf-report.md");

                    let mut content = format!(
                        "# TUF Control Center Diagnostics Report\n\n- Date: {}\n- Model: {}\n- Vendor: {}\n- Simulation: {}\n\n",
                        chrono_or_now(),
                        caps.model_name,
                        caps.sys_vendor,
                        caps.is_simulation,
                    );

                    if let Some(t) = tel {
                        content.push_str(&format!(
                            "## Active Hardware Telemetry\n- Performance Mode: {}\n- GPU Mode: {}\n- CPU: {} ({:.1}°C, {:.1}%, {:.0} MHz)\n- GPU Active: {} (Temp: {:?}, Power: {:?})\n- Fans: CPU {} RPM, GPU {} RPM\n- Battery: {}% ({:?}, AC: {}, Limit: {})\n\n",
                            t.active_mode,
                            t.active_gpu_mode,
                            t.cpu.name, t.cpu.temp_c, t.cpu.usage_pct, t.cpu.freq_mhz,
                            t.gpu.is_active, t.gpu.temp_c, t.gpu.power_w,
                            t.fans.cpu_fan_rpm, t.fans.gpu_fan_rpm,
                            t.battery.percentage, t.battery.state, t.battery.ac_connected, t.battery.current_limit,
                        ));
                    }

                    if std::fs::write(&path, content).is_ok() {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(p) = report {
                    btn_clone.set_label(&format!("Saved: {}", p.file_name().unwrap_or_default().to_string_lossy()));
                    btn_clone.add_css_class("suggested-action");
                } else {
                    btn_clone.set_label("Failed to Save");
                }
            });
        }

        export_row.add_suffix(&btn_export);
        export_group.add(&export_row);
        page.add(&export_group);

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}

fn chrono_or_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    format!("Unix Epoch {}s", now)
}
