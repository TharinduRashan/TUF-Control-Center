use crate::app::BackendHandle;
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use gtk4::{Label, ScrolledWindow, Switch};

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
        touchpad_switch.connect_state_set(move |_, state| {
            let mode = if state { "enabled" } else { "disabled" };
            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.peripherals.touchpad",
                    "send-events",
                    mode,
                ])
                .spawn();
            gtk4::glib::Propagation::Proceed
        });
        touchpad_row.add_suffix(&touchpad_switch);
        group.add(&touchpad_row);

        let mic_row = ActionRow::builder()
            .title("Microphone Capture")
            .subtitle("PipeWire / WirePlumber audio capture input state")
            .build();
        let mic_switch = Switch::builder().active(true).build();
        mic_switch.connect_state_set(move |_, state| {
            let mute_arg = if state { "0" } else { "1" };
            let _ = std::process::Command::new("wpctl")
                .args(["set-mute", "@DEFAULT_AUDIO_SOURCE@", mute_arg])
                .spawn();
            let pactl_arg = if state { "0" } else { "1" };
            let _ = std::process::Command::new("pactl")
                .args(["set-source-mute", "@DEFAULT_SOURCE@", pactl_arg])
                .spawn();
            gtk4::glib::Propagation::Proceed
        });
        mic_row.add_suffix(&mic_switch);
        group.add(&mic_row);

        let fn_row = ActionRow::builder()
            .title("Fn-Lock Switch")
            .subtitle("Swap function keys and media keys")
            .build();
        let fn_label = Label::builder()
            .label("Unsupported on FA506NC")
            .css_classes(["error", "dim-label"])
            .tooltip_text(
                "Hardware WMI returns -65536 on FA506NC; Fn keys are hardware-multiplexed.",
            )
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
