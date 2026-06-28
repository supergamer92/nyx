//! # Nyx Settings Daemon
//!
//! Central configuration service for Nyx OS. Exposes all system settings
//! via D-Bus so the Settings UI, shell, and third-party apps can
//! read/write configuration consistently.
//!
//! Configuration is stored in a SQLite database rather than scattered
//! config files across /etc.

use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("nyx_settingsd=info")
        .init();

    info!("Starting Nyx Settings Daemon...");

    let mut store = SettingsStore::new();

    // Load defaults
    store.set_defaults();

    info!("Settings daemon ready — {} keys loaded", store.len());
    info!("D-Bus interface: org.nyx.Settings");

    // In production, this would register on D-Bus and serve requests
    // For now, just demonstrate the API
    info!("Theme mode: {}", store.get("appearance.theme_mode").unwrap_or("dark"));
    info!("Accent color: {}", store.get("appearance.accent_color").unwrap_or("#7c5cfc"));
    info!("WiFi enabled: {}", store.get("network.wifi_enabled").unwrap_or("true"));

    Ok(())
}

/// In-memory settings store (production uses SQLite)
struct SettingsStore {
    data: HashMap<String, String>,
}

impl SettingsStore {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn set_defaults(&mut self) {
        // Appearance
        self.set("appearance.theme_mode", "dark");
        self.set("appearance.accent_color", "#7c5cfc");
        self.set("appearance.font_size", "14");
        self.set("appearance.scaling", "1.0");
        self.set("appearance.animations", "true");

        // Network
        self.set("network.wifi_enabled", "true");
        self.set("network.bluetooth_enabled", "true");
        self.set("network.airplane_mode", "false");

        // Sound
        self.set("sound.volume", "75");
        self.set("sound.muted", "false");
        self.set("sound.input_device", "default");
        self.set("sound.output_device", "default");

        // Display
        self.set("display.night_light", "false");
        self.set("display.night_light_schedule", "22:00-06:00");

        // Power
        self.set("power.profile", "balanced");
        self.set("power.screen_timeout", "300");
        self.set("power.sleep_timeout", "900");
        self.set("power.lid_close_action", "suspend");

        // Security
        self.set("security.auto_lock", "true");
        self.set("security.lock_timeout", "300");
        self.set("security.firewall_enabled", "true");

        // Updates
        self.set("updates.auto_check", "true");
        self.set("updates.auto_download", "true");
        self.set("updates.auto_install_security", "true");
        self.set("updates.channel", "stable");

        // Developer
        self.set("developer.enabled", "false");
        self.set("developer.aur_enabled", "false");
        self.set("developer.xwayland_enabled", "false");
        self.set("developer.ssh_enabled", "false");
    }
}
