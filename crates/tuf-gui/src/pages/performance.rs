use crate::app::BackendHandle;
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use gtk4::{Button, Label, ScrolledWindow};
use tuf_core::models::PerformanceMode;

#[derive(Clone)]
pub struct PerformancePage {
    pub container: ScrolledWindow,
}

impl PerformancePage {
    pub fn new(backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let group = PreferencesGroup::builder()
            .title("Operating Modes")
            .description("Select a hardware profile matching your acoustic and power needs")
            .build();

        // Silent Row
        let row_silent = ActionRow::builder()
            .title("Silent")
            .subtitle(PerformanceMode::Silent.description())
            .build();
        let btn_silent = Button::with_label("Activate");
        row_silent.add_suffix(&btn_silent);
        group.add(&row_silent);

        // Balanced Row
        let row_balanced = ActionRow::builder()
            .title("Balanced")
            .subtitle(PerformanceMode::Balanced.description())
            .build();
        let btn_balanced = Button::with_label("Activate");
        row_balanced.add_suffix(&btn_balanced);
        group.add(&row_balanced);

        // Turbo Row
        let row_turbo = ActionRow::builder()
            .title("Turbo")
            .subtitle(PerformanceMode::Turbo.description())
            .build();
        let btn_turbo = Button::with_label("Activate");
        row_turbo.add_suffix(&btn_turbo);
        group.add(&row_turbo);

        page.add(&group);

        // AMD Ryzen Platform Power / EPP Group
        let epp_group = PreferencesGroup::builder()
            .title("AMD Ryzen Energy Preference (EPP)")
            .description("Controls Zen 3+ CPU governor bias via amd-pstate")
            .build();

        let epp_row = ActionRow::builder()
            .title("Energy Performance Preference")
            .subtitle("Dynamic EPP bias matching the active thermal profile")
            .build();
        let epp_label = Label::builder()
            .label("balance_performance")
            .css_classes(["dim-label"])
            .build();
        epp_row.add_suffix(&epp_label);
        epp_group.add(&epp_row);
        page.add(&epp_group);

        // Connect button signals
        {
            let b = backend.clone();
            btn_silent.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_performance_mode(PerformanceMode::Silent);
                }
            });
        }
        {
            let b = backend.clone();
            btn_balanced.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_performance_mode(PerformanceMode::Balanced);
                }
            });
        }
        {
            let b = backend.clone();
            btn_turbo.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_performance_mode(PerformanceMode::Turbo);
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
