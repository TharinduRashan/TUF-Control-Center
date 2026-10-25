use gtk4::prelude::*;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;

use crate::window::MainWindow;
use tuf_core::{HardwareBackend, LinuxHardwareBackend, MockHardwareBackend};

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

        let backend: BackendHandle = if simulate {
            Arc::new(Mutex::new(std::boxed::Box::new(MockHardwareBackend::new())))
        } else {
            Arc::new(Mutex::new(
                std::boxed::Box::new(LinuxHardwareBackend::new()),
            ))
        };

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

            let win_ref = Rc::new(main_win);

            // Asynchronous background telemetry polling worker via async-channel
            let (sender, receiver) = async_channel::bounded::<tuf_core::models::SystemTelemetry>(4);

            let w = win_ref.clone();
            glib::spawn_future_local(async move {
                while let Ok(telemetry) = receiver.recv().await {
                    w.update_telemetry(&telemetry);
                }
            });

            // Trigger immediate initial read
            let b_init = backend.clone();
            if let Ok(mut lock) = b_init.lock() {
                if let Ok(initial_telemetry) = lock.read_telemetry() {
                    win_ref.update_telemetry(&initial_telemetry);
                }
            }

            let b = backend.clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(Duration::from_millis(1000));
                    if let Ok(mut lock) = b.lock() {
                        if let Ok(telemetry) = lock.read_telemetry() {
                            if sender.send_blocking(telemetry).is_err() {
                                break; // Window closed or app exiting
                            }
                        }
                    }
                }
            });
        });

        self.app.run_with_args::<String>(&[])
    }
}
