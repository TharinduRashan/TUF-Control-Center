use crate::app::BackendHandle;
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use gtk4::{Box as GtkBox, Button, Orientation, Scale, ScrolledWindow};
use tuf_core::models::{RgbColor, RgbConfig, RgbMode, RgbSpeed};

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

        // Shared RGB state for the lighting page
        let current_rgb = std::sync::Arc::new(std::sync::Mutex::new(RgbConfig::default()));

        // Connect brightness scale
        {
            let b = backend.clone();
            scale.connect_value_changed(move |s| {
                let val = s.value() as u8;
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_keyboard_brightness(val);
                }
            });
        }

        // Helper to bind Mode buttons
        let bind_mode =
            |btn: &Button,
             mode: RgbMode,
             group: Vec<Button>,
             b: BackendHandle,
             state: std::sync::Arc<std::sync::Mutex<RgbConfig>>| {
                let btn_clone = btn.clone();
                btn.connect_clicked(move |_| {
                    for other in &group {
                        other.remove_css_class("suggested-action");
                    }
                    btn_clone.add_css_class("suggested-action");
                    if let Ok(mut s) = state.lock() {
                        s.mode = mode;
                        if let Ok(mut lock) = b.lock() {
                            let _ = lock.set_keyboard_rgb(&s);
                        }
                    }
                });
            };

        let mode_buttons = vec![
            btn_static.clone(),
            btn_breathing.clone(),
            btn_strobing.clone(),
            btn_cycle.clone(),
        ];
        bind_mode(
            &btn_static,
            RgbMode::Static,
            mode_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_mode(
            &btn_breathing,
            RgbMode::Breathing,
            mode_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_mode(
            &btn_strobing,
            RgbMode::Strobing,
            mode_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_mode(
            &btn_cycle,
            RgbMode::ColorCycle,
            mode_buttons,
            backend.clone(),
            current_rgb.clone(),
        );

        // Helper to bind Speed buttons
        let bind_speed =
            |btn: &Button,
             speed: RgbSpeed,
             group: Vec<Button>,
             b: BackendHandle,
             state: std::sync::Arc<std::sync::Mutex<RgbConfig>>| {
                let btn_clone = btn.clone();
                btn.connect_clicked(move |_| {
                    for other in &group {
                        other.remove_css_class("suggested-action");
                    }
                    btn_clone.add_css_class("suggested-action");
                    if let Ok(mut s) = state.lock() {
                        s.speed = speed;
                        if let Ok(mut lock) = b.lock() {
                            let _ = lock.set_keyboard_rgb(&s);
                        }
                    }
                });
            };

        let speed_buttons = vec![btn_slow.clone(), btn_normal.clone(), btn_fast.clone()];
        bind_speed(
            &btn_slow,
            RgbSpeed::Slow,
            speed_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_speed(
            &btn_normal,
            RgbSpeed::Normal,
            speed_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_speed(
            &btn_fast,
            RgbSpeed::Fast,
            speed_buttons,
            backend.clone(),
            current_rgb.clone(),
        );

        // Helper to bind Color buttons
        let bind_color =
            |btn: &Button,
             color: RgbColor,
             group: Vec<Button>,
             b: BackendHandle,
             state: std::sync::Arc<std::sync::Mutex<RgbConfig>>| {
                let btn_clone = btn.clone();
                btn.connect_clicked(move |_| {
                    for other in &group {
                        other.remove_css_class("suggested-action");
                    }
                    btn_clone.add_css_class("suggested-action");
                    if let Ok(mut s) = state.lock() {
                        s.color = color;
                        if let Ok(mut lock) = b.lock() {
                            let _ = lock.set_keyboard_rgb(&s);
                        }
                    }
                });
            };

        let color_buttons = vec![
            btn_orange.clone(),
            btn_red.clone(),
            btn_blue.clone(),
            btn_green.clone(),
            btn_white.clone(),
        ];
        bind_color(
            &btn_orange,
            RgbColor::TUF_ORANGE,
            color_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_color(
            &btn_red,
            RgbColor::RED,
            color_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_color(
            &btn_blue,
            RgbColor::BLUE,
            color_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_color(
            &btn_green,
            RgbColor::GREEN,
            color_buttons.clone(),
            backend.clone(),
            current_rgb.clone(),
        );
        bind_color(
            &btn_white,
            RgbColor::WHITE,
            color_buttons,
            backend.clone(),
            current_rgb.clone(),
        );

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}
