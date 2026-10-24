use gtk4::prelude::*;
use gtk4::{Button, Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use tuf_core::models::BatteryLimit;
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct BatteryPage {
    pub container: ScrolledWindow,
}

impl BatteryPage {
    pub fn new(backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let group = PreferencesGroup::builder()
            .title("Battery Charge Protection")
            .description("Configure hardware charge threshold in the Embedded Controller to reduce battery degradation")
            .build();

        // 60% row
        let row_60 = ActionRow::builder()
            .title("60% — Maximum Lifespan")
            .subtitle(BatteryLimit::MaximumLifespan.description())
            .build();
        let btn_60 = Button::with_label("Set 60%");
        row_60.add_suffix(&btn_60);
        group.add(&row_60);

        // 80% row
        let row_80 = ActionRow::builder()
            .title("80% — Balanced Daily Use (Recommended)")
            .subtitle(BatteryLimit::Balanced.description())
            .build();
        let btn_80 = Button::with_label("Set 80%");
        btn_80.add_css_class("suggested-action");
        row_80.add_suffix(&btn_80);
        group.add(&row_80);

        // 100% row
        let row_100 = ActionRow::builder()
            .title("100% — Full Travel Capacity")
            .subtitle(BatteryLimit::Full.description())
            .build();
        let btn_100 = Button::with_label("Set 100%");
        row_100.add_suffix(&btn_100);
        group.add(&row_100);

        page.add(&group);

        // Health information group
        let health_group = PreferencesGroup::builder()
            .title("Battery Health & Chemistry")
            .description("Hardware telemetry reported by the ACPI battery fuel gauge")
            .build();

        let health_row = ActionRow::builder()
            .title("Estimated Battery Health")
            .subtitle("Full charge capacity compared against design capacity")
            .build();
        let health_label = Label::builder().label("98.5% (Optimal)").build();
        health_row.add_suffix(&health_label);
        health_group.add(&health_row);

        let cycle_row = ActionRow::builder()
            .title("Total Charge Cycles")
            .subtitle("Cumulative full discharge/recharge equivalents")
            .build();
        let cycle_label = Label::builder().label("42 Cycles").build();
        cycle_row.add_suffix(&cycle_label);
        health_group.add(&cycle_row);

        page.add(&health_group);

        // Button clicks
        {
            let b = backend.clone();
            btn_60.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_battery_limit(BatteryLimit::MaximumLifespan);
                }
            });
        }
        {
            let b = backend.clone();
            btn_80.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_battery_limit(BatteryLimit::Balanced);
                }
            });
        }
        {
            let b = backend.clone();
            btn_100.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_battery_limit(BatteryLimit::Full);
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
