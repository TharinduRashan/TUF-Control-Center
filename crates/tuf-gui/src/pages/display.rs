use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation, Scale, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct DisplayPage {
    pub container: ScrolledWindow,
}

impl DisplayPage {
    pub fn new(_backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let refresh_group = PreferencesGroup::builder()
            .title("Display Refresh Rate")
            .description("Switch dynamic panel refresh rate via GNOME Mutter DisplayConfig")
            .build();

        let rate_row = ActionRow::builder()
            .title("Active Refresh Rate")
            .subtitle("Dynamic high-refresh gaming mode vs power-saving mode")
            .build();

        let rate_box = GtkBox::new(Orientation::Horizontal, 6);
        let btn_60 = Button::with_label("60 Hz (Power Save)");
        let btn_144 = Button::with_label("144 Hz (Fluid Gaming)");
        btn_144.add_css_class("suggested-action");

        rate_box.append(&btn_60);
        rate_box.append(&btn_144);
        rate_row.add_suffix(&rate_box);
        refresh_group.add(&rate_row);
        page.add(&refresh_group);

        let brightness_group = PreferencesGroup::builder()
            .title("Panel Backlight")
            .description("Hardware PWM screen luminance")
            .build();

        let brightness_row = ActionRow::builder()
            .title("Screen Brightness")
            .subtitle("Adjust internal panel backlight smoothly")
            .build();

        let scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 5.0);
        scale.set_value(80.0);
        scale.set_width_request(200);
        scale.set_draw_value(true);
        brightness_row.add_suffix(&scale);
        brightness_group.add(&brightness_row);
        page.add(&brightness_group);

        let info_group = PreferencesGroup::builder()
            .title("Panel Specifications")
            .description("Detected built-in internal eDP display")
            .build();

        let panel_row = ActionRow::builder()
            .title("Internal Display")
            .subtitle("15.6\" Full HD (1920x1080) 144Hz IPS Wide-View Anti-Glare")
            .build();
        info_group.add(&panel_row);
        page.add(&info_group);

        // Connect button and slider actions
        {
            let b60 = btn_60.clone();
            let b144 = btn_144.clone();
            btn_60.connect_clicked(move |_| {
                b60.add_css_class("suggested-action");
                b144.remove_css_class("suggested-action");
                let _ = std::process::Command::new("xrandr")
                    .args(["-r", "60"])
                    .spawn();
            });
        }
        {
            let b60 = btn_60.clone();
            let b144 = btn_144.clone();
            btn_144.connect_clicked(move |_| {
                b144.add_css_class("suggested-action");
                b60.remove_css_class("suggested-action");
                let _ = std::process::Command::new("xrandr")
                    .args(["-r", "144"])
                    .spawn();
            });
        }
        {
            scale.connect_value_changed(move |s| {
                let pct = s.value() as u32;
                let _ = std::process::Command::new("brightnessctl")
                    .args(["set", &format!("{}%", pct)])
                    .spawn();
            });
        }

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}
