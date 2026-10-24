use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, ProgressBar};

#[derive(Clone)]
pub struct TelemetryCard {
    pub container: Box,
    pub _title_label: Label,
    pub value_label: Label,
    pub sub_label: Label,
    pub progress: ProgressBar,
}

impl TelemetryCard {
    pub fn new(title: &str, initial_value: &str, initial_sub: &str) -> Self {
        let container = Box::new(Orientation::Vertical, 6);
        container.add_css_class("card");
        container.set_margin_start(6);
        container.set_margin_end(6);
        container.set_margin_top(6);
        container.set_margin_bottom(6);

        let title_label = Label::builder()
            .label(title)
            .xalign(0.0)
            .css_classes(["heading", "dim-label"])
            .build();

        let value_label = Label::builder()
            .label(initial_value)
            .xalign(0.0)
            .css_classes(["title-1"])
            .build();

        let sub_label = Label::builder()
            .label(initial_sub)
            .xalign(0.0)
            .css_classes(["caption", "dim-label"])
            .build();

        let progress = ProgressBar::builder()
            .fraction(0.0)
            .show_text(false)
            .build();

        container.append(&title_label);
        container.append(&value_label);
        container.append(&sub_label);
        container.append(&progress);

        Self {
            container,
            _title_label: title_label,
            value_label,
            sub_label,
            progress,
        }
    }

    pub fn update(&self, value: &str, sub: &str, fraction: f64) {
        self.value_label.set_text(value);
        self.sub_label.set_text(sub);
        self.progress.set_fraction(fraction.clamp(0.0, 1.0));
    }
}
