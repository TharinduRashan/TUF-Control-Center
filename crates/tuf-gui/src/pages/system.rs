use gtk4::{Label, ScrolledWindow, Switch};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct SystemPage {
    pub container: ScrolledWindow,
}

impl SystemPage {
    pub fn new(_backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let group = PreferencesGroup::builder()
            .title("Input & Peripherals")
            .description("Hardware switches and desktop input integrations")
            .build();

        let touchpad_row = ActionRow::builder()
            .title("Touchpad")
            .subtitle("Toggle laptop precision touchpad")
            .build();
        let touchpad_switch = Switch::builder().active(true).build();
        touchpad_row.add_suffix(&touchpad_switch);
        group.add(&touchpad_row);

        let mic_row = ActionRow::builder()
            .title("Microphone Capture")
            .subtitle("PipeWire / WirePlumber audio capture input state")
            .build();
        let mic_switch = Switch::builder().active(true).build();
        mic_row.add_suffix(&mic_switch);
        group.add(&mic_row);

        let fn_row = ActionRow::builder()
            .title("Fn-Lock Switch")
            .subtitle("Swap function keys and media keys")
            .build();
        let fn_label = Label::builder()
            .label("Unsupported on FA506NC")
            .css_classes(["error", "dim-label"])
            .tooltip_text("Hardware WMI returns -65536 on FA506NC; Fn keys are hardware-multiplexed.")
            .build();
        fn_row.add_suffix(&fn_label);
        group.add(&fn_row);

        page.add(&group);

        let audio_group = PreferencesGroup::builder()
            .title("Audio Architecture")
            .description("Preserving Ubuntu's native low-latency sound server")
            .build();

        let audio_row = ActionRow::builder()
            .title("Audio Server")
            .subtitle("PipeWire & WirePlumber Session Manager — 100% Compatible")
            .build();
        audio_group.add(&audio_row);
        page.add(&audio_group);

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self { container }
    }
}
