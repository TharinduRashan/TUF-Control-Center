use gtk4::{Button, Orientation, Scale, ScrolledWindow, Switch};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use tuf_core::AppConfig;
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct SettingsPage {
    pub container: ScrolledWindow,
}

impl SettingsPage {
    pub fn new(_backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let app_group = PreferencesGroup::builder()
            .title("Application Behavior")
            .description("Background service and startup preferences")
            .build();

        let start_min_row = ActionRow::builder()
            .title("Start Minimized")
            .subtitle("Launch in background tray on desktop login")
            .build();
        let start_min_switch = Switch::builder().active(false).build();
        start_min_row.add_suffix(&start_min_switch);
        app_group.add(&start_min_row);

        let tray_row = ActionRow::builder()
            .title("Tray Indicator")
            .subtitle("Display AppIndicator menu icon in GNOME top bar")
            .build();
        let tray_switch = Switch::builder().active(true).build();
        tray_row.add_suffix(&tray_switch);
        app_group.add(&tray_row);

        let poll_row = ActionRow::builder()
            .title("Telemetry Polling Frequency")
            .subtitle("Sensor query interval (ms) — increases to 5000ms on battery")
            .build();
        let poll_scale = Scale::with_range(Orientation::Horizontal, 500.0, 5000.0, 250.0);
        poll_scale.set_value(1000.0);
        poll_scale.set_width_request(180);
        poll_scale.set_draw_value(true);
        poll_row.add_suffix(&poll_scale);
        app_group.add(&poll_row);

        page.add(&app_group);

        let config_group = PreferencesGroup::builder()
            .title("Configuration Management")
            .description("Persistent TOML configuration file")
            .build();

        let path_row = ActionRow::builder()
            .title("Config Location")
            .subtitle(AppConfig::default_path().to_string_lossy().to_string())
            .build();
        config_group.add(&path_row);

        let reset_row = ActionRow::builder()
            .title("Factory Defaults")
            .subtitle("Reset all operating profiles, battery thresholds, and RGB settings")
            .build();
        let btn_reset = Button::with_label("Reset Defaults");
        btn_reset.add_css_class("destructive-action");
        reset_row.add_suffix(&btn_reset);
        config_group.add(&reset_row);

        page.add(&config_group);

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}
