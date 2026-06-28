//! # Compositor State
//!
//! The central state machine for the Nyx compositor.
//! Holds all managed windows, workspaces, outputs, and input state.

use crate::animation::AnimationEngine;
use crate::workspace::WorkspaceManager;
use crate::window::WindowManager;
use crate::input::InputManager;
use crate::output::OutputManager;

use tracing::info;

/// The master compositor state
pub struct NyxCompositorState {
    /// Manages all workspaces and their window assignments
    pub workspaces: WorkspaceManager,

    /// Manages individual window properties (geometry, decorations, etc.)
    pub windows: WindowManager,

    /// Handles keyboard, mouse, touchpad input and gesture recognition
    pub input: InputManager,

    /// Manages physical displays (resolution, scaling, arrangement)
    pub outputs: OutputManager,

    /// Spring-physics animation engine for smooth transitions
    pub animations: AnimationEngine,

    /// Whether the compositor is running
    pub running: bool,

    /// Frame counter for performance tracking
    pub frame_count: u64,

    /// Compositor start time
    pub start_time: std::time::Instant,
}

impl NyxCompositorState {
    /// Create a new compositor state with default configuration
    pub fn new() -> anyhow::Result<Self> {
        info!("Creating compositor state...");

        let state = Self {
            workspaces: WorkspaceManager::new(4), // Start with 4 workspaces
            windows: WindowManager::new(),
            input: InputManager::new(),
            outputs: OutputManager::new(),
            animations: AnimationEngine::new(),
            running: true,
            frame_count: 0,
            start_time: std::time::Instant::now(),
        };

        info!("Compositor state ready");
        info!(
            "  Workspaces: {}",
            state.workspaces.count()
        );

        Ok(state)
    }

    /// Main event loop tick
    pub fn tick(&mut self, dt: f32) {
        // 1. Process input events
        self.input.process_pending();

        // 2. Update animations
        self.animations.update(dt);

        // 3. Apply animation values to windows
        for window in self.windows.all_mut() {
            if let Some(anim) = self.animations.get_window_animation(window.id) {
                window.apply_animation(anim);
            }
        }

        // 4. Mark frame
        self.frame_count += 1;
    }

    /// Run the compositor event loop
    pub fn run(&mut self) -> anyhow::Result<()> {
        info!("Entering compositor event loop");

        // In a real implementation, this would be driven by calloop
        // with Smithay's event sources (Wayland clients, input, DRM, etc.)
        //
        // For the initial skeleton, we simulate the structure:

        let target_fps = 60.0_f32;
        let frame_time = std::time::Duration::from_secs_f32(1.0 / target_fps);

        while self.running {
            let frame_start = std::time::Instant::now();

            // Tick the compositor
            self.tick(1.0 / target_fps);

            // Sleep for remaining frame time
            let elapsed = frame_start.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }

            // Log FPS every 5 seconds
            if self.frame_count % (target_fps as u64 * 5) == 0 {
                let uptime = self.start_time.elapsed().as_secs();
                info!(
                    "Compositor uptime: {}s, frames: {}, windows: {}",
                    uptime,
                    self.frame_count,
                    self.windows.count(),
                );
            }

            // Exit after 1 frame in skeleton mode
            if self.frame_count >= 1 {
                info!("Skeleton compositor: exiting after first frame");
                info!("(Full event loop requires Linux + Wayland environment)");
                self.running = false;
            }
        }

        info!("Compositor shut down cleanly");
        Ok(())
    }
}
