use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation, Scale, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use tuf_core::models::{RgbColor, RgbConfig, RgbMode, RgbSpeed};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct LightingPage {
    pub container: ScrolledWindow,
}

impl LightingPage {
    pub fn new(backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        // Brightness group
        let brightness_group = PreferencesGroup::builder()
            .title("Keyboard Backlight Brightness")
            .description("Hardware LED intensity (0 = Off, 3 = Maximum)")
            .build();

        let brightness_row = ActionRow::builder()
            .title("Backlight Level")
            .subtitle("sysfs asus::kbd_backlight/brightness")
            .build();

        let scale = Scale::with_range(Orientation::Horizontal, 0.0, 3.0, 1.0);
        scale.set_value(3.0);
        scale.set_width_request(180);
        scale.set_draw_value(true);
        scale.set_digits(0);
        brightness_row.add_suffix(&scale);
        brightness_group.add(&brightness_row);
        page.add(&brightness_group);

        // Lighting Effects Group
        let effects_group = PreferencesGroup::builder()
            .title("TUF Aura Lighting Effects")
            .description("Single-zone RGB animation modes via ASUS USB HID protocol")
            .build();

        let mode_row = ActionRow::builder()
            .title("Animation Mode")
            .subtitle("Firmware pattern engine")
            .build();

        let mode_box = GtkBox::new(Orientation::Horizontal, 6);
        let btn_static = Button::with_label("Static");
        let btn_breathing = Button::with_label("Breathing");
        let btn_strobing = Button::with_label("Strobing");
        let btn_cycle = Button::with_label("Color Cycle");

        btn_static.add_css_class("suggested-action");

        mode_box.append(&btn_static);
        mode_box.append(&btn_breathing);
        mode_box.append(&btn_strobing);
        mode_box.append(&btn_cycle);
        mode_row.add_suffix(&mode_box);
        effects_group.add(&mode_row);

        // Speed Row
        let speed_row = ActionRow::builder()
            .title("Animation Speed")
            .subtitle("Rate of pulse or transition")
            .build();

        let speed_box = GtkBox::new(Orientation::Horizontal, 6);
        let btn_slow = Button::with_label("Slow");
        let btn_normal = Button::with_label("Normal");
        let btn_fast = Button::with_label("Fast");
        btn_normal.add_css_class("suggested-action");

        speed_box.append(&btn_slow);
        speed_box.append(&btn_normal);
        speed_box.append(&btn_fast);
        speed_row.add_suffix(&speed_box);
        effects_group.add(&speed_row);

        // Palette Swatches
        let color_row = ActionRow::builder()
            .title("Color Swatches")
            .subtitle("Preset TUF Gaming and custom colors")
            .build();

        let color_box = GtkBox::new(Orientation::Horizontal, 6);
        let btn_orange = Button::with_label("TUF Orange");
        let btn_red = Button::with_label("Crimson");
        let btn_blue = Button::with_label("Cyan");
        let btn_green = Button::with_label("Green");
        let btn_white = Button::with_label("White");

        btn_orange.add_css_class("suggested-action");

        color_box.append(&btn_orange);
        color_box.append(&btn_red);
        color_box.append(&btn_blue);
        color_box.append(&btn_green);
        color_box.append(&btn_white);
        color_row.add_suffix(&color_box);
        effects_group.add(&color_row);

        page.add(&effects_group);

        // Connect signals
        {
            let b = backend.clone();
            scale.connect_value_changed(move |s| {
                let val = s.value() as u8;
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_keyboard_brightness(val);
                }
            });
        }
        {
            let b = backend.clone();
            btn_orange.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let cfg = RgbConfig {
                        mode: RgbMode::Static,
                        color: RgbColor::TUF_ORANGE,
                        speed: RgbSpeed::Normal,
                        brightness: 3,
                    };
                    let _ = lock.set_keyboard_rgb(&cfg);
                }
            });
        }
        {
            let b = backend.clone();
            btn_red.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let cfg = RgbConfig {
                        mode: RgbMode::Static,
                        color: RgbColor::RED,
                        speed: RgbSpeed::Normal,
                        brightness: 3,
                    };
                    let _ = lock.set_keyboard_rgb(&cfg);
                }
            });
        }

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}
