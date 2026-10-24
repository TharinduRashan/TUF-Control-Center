use std::sync::{Arc, Mutex};
use std::time::Duration;
use gtk4::prelude::*;
use tracing::info;

use tuf_core::{HardwareBackend, MockHardwareBackend};
use crate::window::MainWindow;

pub type BackendHandle = Arc<Mutex<std::boxed::Box<dyn HardwareBackend>>>;

pub struct TufApplication {
    pub app: adw::Application,
    backend: BackendHandle,
    simulate: bool,
}

impl TufApplication {
    pub fn new(simulate: bool) -> Self {
        let app = adw::Application::builder()
            .application_id("org.tuf.ControlCenter")
            .build();

        let backend: BackendHandle =
            Arc::new(Mutex::new(std::boxed::Box::new(MockHardwareBackend::new())));

        Self {
            app,
            backend,
            simulate,
        }
    }

    pub fn run(self) -> gtk4::glib::ExitCode {
        let backend = self.backend.clone();
        let simulate = self.simulate;

        self.app.connect_activate(move |app| {
            info!("TUF Control Center application activated.");
            let main_win = MainWindow::new(app, backend.clone(), simulate);
            main_win.window.present();

            let win_ref = Arc::new(main_win);

            // Periodic telemetry polling on GTK main context
            let b = backend.clone();
            let w = win_ref.clone();

            glib::timeout_add_local(Duration::from_millis(1000), move || {
                if let Ok(mut lock) = b.lock() {
                    if let Ok(telemetry) = lock.read_telemetry() {
                        w.update_telemetry(&telemetry);
                    }
                }
                glib::ControlFlow::Continue
            });
        });

        self.app.run()
    }
}
