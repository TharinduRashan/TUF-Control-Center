extern crate libadwaita as adw;

mod app;
mod pages;
mod widgets;
mod window;

use clap::Parser;
use tracing::info;
use crate::app::TufApplication;

#[derive(Parser, Debug)]
#[command(
    name = "tuf-control-center",
    author = "TUF Control Center Contributors",
    version = tuf_core::version(),
    about = "Linux-native G-Helper clone for ASUS TUF and ROG gaming laptops"
)]
struct Args {
    /// Run with synthetic simulated hardware (no root or ASUS hardware required)
    #[arg(long, env = "TUF_SIMULATE")]
    simulate: bool,

    /// Start minimized to background / system tray
    #[arg(long)]
    minimized: bool,
}

fn main() -> gtk4::glib::ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();

    info!(
        "Launching TUF Control Center GUI v{} (Simulate={}, Minimized={})",
        tuf_core::version(),
        args.simulate,
        args.minimized
    );

    let app = TufApplication::new(args.simulate);
    app.run()
}
