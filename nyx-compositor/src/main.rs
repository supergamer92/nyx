//! # Nyx Compositor
//!
//! The Wayland compositor for Nyx OS, built on Smithay.
//!
//! This is the core of the display server — it manages windows, inputs,
//! outputs, and renders everything to the screen via wgpu/Vulkan.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  NyxCompositor (main state)      │
//! │  ├── InputManager                │
//! │  ├── OutputManager               │
//! │  ├── WorkspaceManager            │
//! │  ├── WindowManager               │
//! │  ├── AnimationEngine             │
//! │  └── Renderer (wgpu)             │
//! └──────────────────────────────────┘
//! ```
//!
//! ## Running
//!
//! On Linux with Wayland or X11 (nested):
//! ```bash
//! cargo run -p nyx-compositor
//! ```

mod state;
mod renderer;
mod input;
mod output;
mod animation;
mod workspace;
mod window;

use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("nyx_compositor=info")),
        )
        .init();

    info!("╔══════════════════════════════════════╗");
    info!("║         Nyx Compositor v0.1.0        ║");
    info!("╚══════════════════════════════════════╝");

    // Detect backend: nested (winit) or direct (DRM/KMS)
    let backend = detect_backend();
    info!("Using backend: {:?}", backend);

    match backend {
        Backend::Winit => {
            info!("Starting in nested/windowed mode (development)");
            // In development, run inside an existing desktop as a window
            run_winit_backend()
        }
        Backend::Drm => {
            info!("Starting in direct rendering mode (production)");
            // On real hardware, take over the display
            run_drm_backend()
        }
    }
}

#[derive(Debug)]
enum Backend {
    /// Nested mode — runs inside an existing Wayland/X11 session
    Winit,
    /// Direct mode — takes over DRM/KMS for real hardware
    Drm,
}

fn detect_backend() -> Backend {
    // If WAYLAND_DISPLAY or DISPLAY is set, we're inside an existing session
    if std::env::var("WAYLAND_DISPLAY").is_ok() || std::env::var("DISPLAY").is_ok() {
        Backend::Winit
    } else {
        Backend::Drm
    }
}

fn run_winit_backend() -> anyhow::Result<()> {
    info!("Initializing Winit backend...");

    // Create the compositor state
    let mut compositor = state::NyxCompositorState::new()?;

    info!("Compositor state initialized");
    info!("Starting event loop...");

    // The event loop would be driven by calloop + Smithay
    // For now, log that we've reached this point
    compositor.run()
}

fn run_drm_backend() -> anyhow::Result<()> {
    info!("Initializing DRM backend...");
    info!("DRM backend not yet implemented — use Winit (nested) mode for development");
    info!("Set WAYLAND_DISPLAY or DISPLAY env var to force nested mode");
    Ok(())
}
