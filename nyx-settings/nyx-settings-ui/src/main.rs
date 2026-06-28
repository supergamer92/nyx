//! # Nyx Settings UI
//!
//! The graphical Settings application. Communicates with nyx-settingsd
//! over D-Bus to read/write system configuration.
//!
//! Stub — full implementation uses sidebar navigation + per-section views.

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("nyx_settings_ui=info")
        .init();

    tracing::info!("Nyx Settings UI — stub");
    tracing::info!("Full implementation will use nyx-widgets sidebar + settings panels");
}
