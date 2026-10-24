use gtk4::prelude::*;
use gtk4::{Button, Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use tuf_core::models::{BatteryLimit, SystemTelemetry};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct BatteryPage {
    pub container: ScrolledWindow,
    btn_60: Button,
    btn_80: Button,
    btn_100: Button,
    health_label: Label,
    cycle_label: Label,
    status_label: Label,
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

        let status_row = ActionRow::builder()
            .title("Current Power Status")
            .subtitle("Real-time charge percentage, wattage, and state")
            .build();
        let status_label = Label::builder().label("--% (Reading...)").build();
        status_row.add_suffix(&status_label);
        health_group.add(&status_row);

        let health_row = ActionRow::builder()
            .title("Estimated Battery Health")
            .subtitle("Full charge capacity compared against design capacity")
            .build();
        let health_label = Label::builder().label("Reading...").build();
        health_row.add_suffix(&health_label);
        health_group.add(&health_row);

        let cycle_row = ActionRow::builder()
            .title("Total Charge Cycles")
            .subtitle("Cumulative full discharge/recharge equivalents")
            .build();
        let cycle_label = Label::builder().label("Reading...").build();
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

        Self {
            container,
            btn_60,
            btn_80,
            btn_100,
            health_label,
            cycle_label,
            status_label,
        }
    }

    pub fn update_telemetry(&self, telemetry: &SystemTelemetry) {
        let status_text = format!(
            "{}% [{}]{}",
            telemetry.battery.percentage,
            telemetry.battery.state,
            if telemetry.battery.power_w.abs() > 0.05 {
                format!("  |  {:.1}W", telemetry.battery.power_w.abs())
            } else {
                String::new()
            }
        );
        self.status_label.set_text(&status_text);

        if let Some(health) = telemetry.battery.health_pct {
            self.health_label.set_text(&format!("{:.1}% (Optimal)", health));
        } else {
            self.health_label.set_text("N/A (Virtual / AC Only)");
        }

        if let Some(cycles) = telemetry.battery.cycle_count {
            self.cycle_label.set_text(&format!("{} Cycles", cycles));
        } else {
            self.cycle_label.set_text("Not Reported by ACPI");
        }

        // Highlight selected button
        let set_active = |btn: &Button, active: bool| {
            if active {
                btn.add_css_class("suggested-action");
            } else {
                btn.remove_css_class("suggested-action");
            }
        };
        set_active(&self.btn_60, telemetry.battery.current_limit == BatteryLimit::MaximumLifespan);
        set_active(&self.btn_80, telemetry.battery.current_limit == BatteryLimit::Balanced);
        set_active(&self.btn_100, telemetry.battery.current_limit == BatteryLimit::Full);
    }
}
