use gtk4::{Button, Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct DiagnosticsPage {
    pub container: ScrolledWindow,
}

impl DiagnosticsPage {
    pub fn new(_backend: BackendHandle) -> Self {
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
            .subtitle("Generates a markdown diagnostics report")
            .build();
        let btn_export = Button::with_label("Export Report");
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
