//! # Nyx Auth
//!
//! Authentication service and Polkit agent for Nyx OS.
//! Handles:
//! - Password authentication
//! - Fingerprint (fprintd integration)
//! - Polkit authorization prompts (styled Nyx dialogs)
//! - Keyring/secret service

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("nyx_auth=info")
        .init();

    tracing::info!("Nyx Auth Agent starting...");
    tracing::info!("Polkit agent: org.nyx.AuthAgent");
    tracing::info!("Supports: password, fingerprint (fprintd)");

    // In production: register as polkit agent and handle auth requests
    // with styled Nyx modal dialogs instead of generic GTK/Qt prompts
    Ok(())
}
