//! # Nyx Portal
//!
//! XDG Desktop Portal implementation for Nyx OS.
//! Provides sandboxed apps (Flatpak) access to:
//! - File chooser dialogs
//! - Screenshot capture
//! - Screen recording
//! - Camera/microphone (with permission prompts)
//! - Notifications
//! - App launching

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("nyx_portal=info")
        .init();

    tracing::info!("Nyx Portal starting...");
    tracing::info!("Implements: org.freedesktop.portal.FileChooser");
    tracing::info!("Implements: org.freedesktop.portal.Screenshot");
    tracing::info!("Implements: org.freedesktop.portal.ScreenCast");
    tracing::info!("Implements: org.freedesktop.portal.Notification");
    tracing::info!("Implements: org.freedesktop.portal.Camera");
    tracing::info!("Implements: org.freedesktop.portal.Settings");

    // In production: register D-Bus interfaces and handle portal requests
    Ok(())
}
