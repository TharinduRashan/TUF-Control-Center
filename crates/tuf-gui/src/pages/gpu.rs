use gtk4::prelude::*;
use gtk4::{Button, Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use tuf_core::models::GpuMode;
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct GpuPage {
    pub container: ScrolledWindow,
}

impl GpuPage {
    pub fn new(backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let group = PreferencesGroup::builder()
            .title("GPU Mode Switching")
            .description("Configure discrete graphics power routing and rendering offload")
            .build();

        // Hybrid (Optimus) Row
        let row_hybrid = ActionRow::builder()
            .title("Hybrid (Optimus / PRIME)")
            .subtitle(GpuMode::Hybrid.description())
            .build();
        let btn_hybrid = Button::with_label("Select Hybrid");
        btn_hybrid.add_css_class("suggested-action");
        row_hybrid.add_suffix(&btn_hybrid);
        group.add(&row_hybrid);

        // Integrated (Eco) Row
        let row_eco = ActionRow::builder()
            .title("Integrated (Eco Mode)")
            .subtitle(GpuMode::Integrated.description())
            .build();
        let btn_eco = Button::with_label("Select Eco");
        row_eco.add_suffix(&btn_eco);
        group.add(&row_eco);

        // Dedicated / Ultimate (MUX) Row
        let row_dedicated = ActionRow::builder()
            .title("Ultimate (Dedicated MUX)")
            .subtitle("Direct display connection via hardware MUX switch")
            .build();
        let unsupported_badge = Label::builder()
            .label("Unsupported on FA506NC")
            .css_classes(["error", "dim-label"])
            .tooltip_text("ASUS TUF Gaming A15 FA506NC routes display signals through the AMD iGPU without an Advanced Optimus hardware MUX chip.")
            .build();
        row_dedicated.add_suffix(&unsupported_badge);
        group.add(&row_dedicated);

        page.add(&group);

        // Architecture Information Group
        let info_group = PreferencesGroup::builder()
            .title("Hardware Details")
            .description("Detected graphics processors and runtime power states")
            .build();

        let igpu_row = ActionRow::builder()
            .title("Integrated Graphics")
            .subtitle("AMD Radeon 660M (RDNA 2) — Drives internal 144Hz panel")
            .build();
        info_group.add(&igpu_row);

        let dgpu_row = ActionRow::builder()
            .title("Discrete Graphics")
            .subtitle("NVIDIA GeForce RTX 3050 Laptop GPU — PCIe Runtime D3cold")
            .build();
        info_group.add(&dgpu_row);

        page.add(&info_group);

        // Button clicks
        {
            let b = backend.clone();
            btn_hybrid.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_gpu_mode(GpuMode::Hybrid);
                }
            });
        }
        {
            let b = backend.clone();
            btn_eco.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_gpu_mode(GpuMode::Integrated);
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
