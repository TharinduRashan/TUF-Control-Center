mod dbus;
mod hid;
mod polkit;
mod safety;
mod sysfs;

use std::sync::Arc;
use clap::Parser;
use tracing::{info, warn};
use zbus::connection::Builder;

use crate::dbus::ControlCenterDaemon;
use crate::hid::TufHidController;
use crate::polkit::PolkitChecker;
use crate::safety::SafetyWatchdog;
use crate::sysfs::SysfsController;

#[derive(Parser, Debug)]
#[command(
    name = "tuf-daemon",
    author = "TUF Control Center Contributors",
    version = tuf_core::version(),
    about = "Privileged system daemon for ASUS TUF & ROG hardware control"
)]
struct Args {
    /// Enable hardware simulation mode (no root or ASUS hardware required)
    #[arg(long, env = "TUF_SIMULATE")]
    simulate: bool,

    /// Disable PolicyKit authentication checks (for debugging)
    #[arg(long)]
    no_polkit: bool,

    /// Bind to user session D-Bus instead of system D-Bus (for development)
    #[arg(long)]
    session_bus: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();

    info!(
        "Starting TUF Control Center Daemon v{} (Simulate={}, NoPolkit={}, SessionBus={})",
        tuf_core::version(),
        args.simulate,
        args.no_polkit,
        args.session_bus
    );

    let sysfs = Arc::new(SysfsController::new(args.simulate));
    let hid = Arc::new(TufHidController::new(args.simulate));
    let polkit = Arc::new(PolkitChecker::new(!args.no_polkit));

    let watchdog = SafetyWatchdog::new();
    watchdog.setup_signal_handlers(sysfs.clone());

    let daemon = ControlCenterDaemon::new(
        sysfs.clone(),
        hid.clone(),
        polkit.clone(),
        args.simulate,
    );

    let conn = if args.session_bus || args.simulate {
        info!("Binding D-Bus interface to Session Bus...");
        Builder::session()?
            .name("org.tuf.ControlCenter")?
            .serve_at("/org/tuf/ControlCenter", daemon)?
            .build()
            .await?
    } else {
        info!("Binding D-Bus interface to System Bus (org.tuf.ControlCenter)...");
        match Builder::system() {
            Ok(builder) => {
                builder
                    .name("org.tuf.ControlCenter")?
                    .serve_at("/org/tuf/ControlCenter", daemon)?
                    .build()
                    .await?
            }
            Err(e) => {
                warn!("Could not bind to system bus ({e}). Falling back to session bus.");
                Builder::session()?
                    .name("org.tuf.ControlCenter")?
                    .serve_at("/org/tuf/ControlCenter", daemon)?
                    .build()
                    .await?
            }
        }
    };

    info!("TUF Control Center Daemon is active and ready on D-Bus.");

    // Keep running until signal
    std::future::pending::<()>().await;

    drop(conn);
    Ok(())
}
