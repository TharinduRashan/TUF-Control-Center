use gtk4::{Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct FansPage {
    pub container: ScrolledWindow,
}

impl FansPage {
    pub fn new(_backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let group = PreferencesGroup::builder()
            .title("Dual Fan Telemetry & Curves")
            .description("Real-time cooling performance and acoustic profiles")
            .build();

        let cpu_fan_row = ActionRow::builder()
            .title("CPU Blower Fan")
            .subtitle("Dynamic RPM governed by AMD Ryzen Tctl temperature")
            .build();
        let cpu_rpm_label = Label::builder().label("Active (Firmware Regulated)").build();
        cpu_fan_row.add_suffix(&cpu_rpm_label);
        group.add(&cpu_fan_row);

        let gpu_fan_row = ActionRow::builder()
            .title("GPU Blower Fan")
            .subtitle("Dynamic RPM governed by NVIDIA GA107 hotspot temperature")
            .build();
        let gpu_rpm_label = Label::builder().label("Active (Firmware Regulated)").build();
        gpu_fan_row.add_suffix(&gpu_rpm_label);
        group.add(&gpu_fan_row);

        page.add(&group);

        let curves_group = PreferencesGroup::builder()
            .title("Firmware Curve Mappings")
            .description("On ASUS TUF models, fan curves are controlled by Embedded Controller firmware modes")
            .build();

        let silent_curve = ActionRow::builder()
            .title("Silent Curve")
            .subtitle("Fans off up to 55°C; acoustic target capped under 32 dBA")
            .build();
        curves_group.add(&silent_curve);

        let balanced_curve = ActionRow::builder()
            .title("Balanced Curve")
            .subtitle("Progressive linear scaling up to 4200 RPM at 85°C")
            .build();
        curves_group.add(&balanced_curve);

        let turbo_curve = ActionRow::builder()
            .title("Turbo Overboost Curve")
            .subtitle("Aggressive ramp-up reaching peak 5000+ RPM under sustained load")
            .build();
        curves_group.add(&turbo_curve);

        page.add(&curves_group);

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}
