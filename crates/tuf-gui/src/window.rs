use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation};
use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar, ViewStack, ViewSwitcher, ViewSwitcherBar};

use tuf_core::models::SystemTelemetry;
use crate::app::BackendHandle;
use crate::pages::{
    BatteryPage, DashboardPage, DiagnosticsPage, DisplayPage, FansPage, GpuPage, LightingPage,
    PerformancePage, SettingsPage, SystemPage,
};

pub struct MainWindow {
    pub window: ApplicationWindow,
    pub dashboard_page: DashboardPage,
    pub _stack: ViewStack,
}

impl MainWindow {
    pub fn new(
        app: &adw::Application,
        backend: BackendHandle,
        is_simulation: bool,
    ) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("TUF Control Center")
            .default_width(880)
            .default_height(680)
            .build();

        let header = HeaderBar::new();

        let title_box = GtkBox::new(Orientation::Vertical, 2);
        let title_label = Label::builder()
            .label("TUF Control Center")
            .css_classes(["title"])
            .build();
        let subtitle_label = Label::builder()
            .label("ASUS TUF Gaming A15 FA506NC")
            .css_classes(["subtitle"])
            .build();
        title_box.append(&title_label);
        title_box.append(&subtitle_label);

        if is_simulation {
            let sim_badge = Label::builder()
                .label("SIMULATION MODE")
                .css_classes(["warning", "caption"])
                .build();
            header.pack_start(&sim_badge);
        }

        header.set_title_widget(Some(&title_box));

        let stack = ViewStack::new();

        // 1. Dashboard
        let dashboard_page = DashboardPage::new(backend.clone());
        let dash_page = stack.add_titled(&dashboard_page.container, Some("dashboard"), "Dashboard");
        dash_page.set_icon_name(Some("speedometer-symbolic"));

        // 2. Performance
        let performance_page = PerformancePage::new(backend.clone());
        let perf_page = stack.add_titled(&performance_page.container, Some("performance"), "Performance");
        perf_page.set_icon_name(Some("power-profile-balanced-symbolic"));

        // 3. GPU
        let gpu_page = GpuPage::new(backend.clone());
        let g_page = stack.add_titled(&gpu_page.container, Some("gpu"), "GPU");
        g_page.set_icon_name(Some("video-display-symbolic"));

        // 4. Fans
        let fans_page = FansPage::new(backend.clone());
        let f_page = stack.add_titled(&fans_page.container, Some("fans"), "Fans");
        f_page.set_icon_name(Some("weather-windy-symbolic"));

        // 5. Battery
        let battery_page = BatteryPage::new(backend.clone());
        let b_page = stack.add_titled(&battery_page.container, Some("battery"), "Battery");
        b_page.set_icon_name(Some("battery-symbolic"));

        // 6. Display
        let display_page = DisplayPage::new(backend.clone());
        let disp_page = stack.add_titled(&display_page.container, Some("display"), "Display");
        disp_page.set_icon_name(Some("preferences-desktop-display-symbolic"));

        // 7. Lighting
        let lighting_page = LightingPage::new(backend.clone());
        let light_page = stack.add_titled(&lighting_page.container, Some("lighting"), "Lighting");
        light_page.set_icon_name(Some("preferences-color-symbolic"));

        // 8. System
        let system_page = SystemPage::new(backend.clone());
        let sys_page = stack.add_titled(&system_page.container, Some("system"), "System");
        sys_page.set_icon_name(Some("input-keyboard-symbolic"));

        // 9. Settings
        let settings_page = SettingsPage::new(backend.clone());
        let set_page = stack.add_titled(&settings_page.container, Some("settings"), "Settings");
        set_page.set_icon_name(Some("emblem-system-symbolic"));

        // 10. Diagnostics
        let diagnostics_page = DiagnosticsPage::new(backend.clone());
        let diag_page = stack.add_titled(&diagnostics_page.container, Some("diagnostics"), "Diagnostics");
        diag_page.set_icon_name(Some("dialog-information-symbolic"));

        let switcher_bar = ViewSwitcherBar::builder()
            .stack(&stack)
            .reveal(true)
            .build();

        let top_switcher = ViewSwitcher::builder()
            .stack(&stack)
            .policy(adw::ViewSwitcherPolicy::Wide)
            .build();

        header.pack_end(&top_switcher);

        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.append(&header);
        main_box.append(&stack);
        main_box.append(&switcher_bar);

        stack.set_vexpand(true);
        window.set_content(Some(&main_box));

        Self {
            window,
            dashboard_page,
            _stack: stack,
        }
    }

    pub fn update_telemetry(&self, telemetry: &SystemTelemetry) {
        self.dashboard_page.update_telemetry(telemetry);
    }
}
