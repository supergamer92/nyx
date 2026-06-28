//! # Nyx Session Manager
//!
//! Manages the user session lifecycle:
//! - Login greeter (graphical, replaces LightDM/GDM/SDDM)
//! - Session startup (compositor → shell → autostart apps)
//! - Lock screen activation
//! - Suspend / hibernate
//! - Logout / shutdown / reboot

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("nyx_session=info")
        .init();

    tracing::info!("Nyx Session Manager starting...");
    tracing::info!("Session lifecycle: greeter → login → compositor → shell → desktop");

    // Session startup sequence:
    // 1. Display greeter (login screen)
    // 2. Authenticate user (password / fingerprint / face)
    // 3. Start nyx-compositor
    // 4. Start nyx-shell (layer-shell client)
    // 5. Start nyx-settingsd
    // 6. Start nyx-portal
    // 7. Start nyx-hw
    // 8. Run user autostart apps
    // 9. Session is active

    Ok(())
}
