//! # Animation Engine
//!
//! Spring-physics animation system for natural, 60fps+ UI motion.
//! Every visual transition in Nyx goes through this engine.

use std::collections::HashMap;
use crate::window::WindowId;

/// A spring-based animation value
#[derive(Debug, Clone)]
pub struct SpringAnimation {
    /// Current value
    pub value: f32,
    /// Target value
    pub target: f32,
    /// Current velocity
    pub velocity: f32,
    /// Spring stiffness (higher = snappier)
    pub stiffness: f32,
    /// Damping ratio (higher = less bounce)
    pub damping: f32,
    /// Mass (higher = more inertia)
    pub mass: f32,
    /// Whether the animation has settled
    pub settled: bool,
}

impl SpringAnimation {
    pub fn new(initial: f32, target: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            value: initial,
            target,
            velocity: 0.0,
            stiffness,
            damping,
            mass: 1.0,
            settled: false,
        }
    }

    /// Create with the default Nyx spring feel
    pub fn default_spring(initial: f32, target: f32) -> Self {
        Self::new(initial, target, 300.0, 25.0)
    }

    /// Create a snappy spring (for toggles, buttons)
    pub fn snappy(initial: f32, target: f32) -> Self {
        Self::new(initial, target, 400.0, 30.0)
    }

    /// Create a gentle spring (for large transitions)
    pub fn gentle(initial: f32, target: f32) -> Self {
        Self::new(initial, target, 200.0, 22.0)
    }

    /// Create a bouncy spring (for dock magnification)
    pub fn bouncy(initial: f32, target: f32) -> Self {
        Self::new(initial, target, 350.0, 18.0)
    }

    /// Update the spring physics for one time step
    pub fn update(&mut self, dt: f32) {
        if self.settled {
            return;
        }

        // Spring force: F = -k * (x - target) - d * v
        let displacement = self.value - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.mass;

        // Semi-implicit Euler integration
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        // Check if settled (close enough and slow enough)
        if displacement.abs() < 0.001 && self.velocity.abs() < 0.001 {
            self.value = self.target;
            self.velocity = 0.0;
            self.settled = true;
        }
    }

    /// Change the target value (re-activates the animation)
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
        self.settled = false;
    }

    /// Instantly set value and target (no animation)
    pub fn set_immediate(&mut self, value: f32) {
        self.value = value;
        self.target = value;
        self.velocity = 0.0;
        self.settled = true;
    }
}

/// Animation values applied to a window
#[derive(Debug, Clone)]
pub struct WindowAnimation {
    pub x: SpringAnimation,
    pub y: SpringAnimation,
    pub width: SpringAnimation,
    pub height: SpringAnimation,
    pub opacity: SpringAnimation,
    pub scale: SpringAnimation,
    pub corner_radius: SpringAnimation,
}

impl WindowAnimation {
    /// Create a new window animation at the given position
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x: SpringAnimation::default_spring(x, x),
            y: SpringAnimation::default_spring(y, y),
            width: SpringAnimation::default_spring(w, w),
            height: SpringAnimation::default_spring(h, h),
            opacity: SpringAnimation::snappy(1.0, 1.0),
            scale: SpringAnimation::snappy(1.0, 1.0),
            corner_radius: SpringAnimation::snappy(12.0, 12.0),
        }
    }

    /// Animate to a new position/size
    pub fn animate_to(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x.set_target(x);
        self.y.set_target(y);
        self.width.set_target(w);
        self.height.set_target(h);
    }

    /// Play window open animation (scale up from 0.9 + fade in)
    pub fn play_open(&mut self) {
        self.opacity.value = 0.0;
        self.opacity.set_target(1.0);
        self.scale.value = 0.92;
        self.scale.set_target(1.0);
    }

    /// Play window close animation (scale down + fade out)
    pub fn play_close(&mut self) {
        self.opacity.set_target(0.0);
        self.scale.set_target(0.92);
    }

    /// Play minimize animation (scale down + slide to dock)
    pub fn play_minimize(&mut self, dock_x: f32, dock_y: f32) {
        self.opacity.set_target(0.0);
        self.scale.set_target(0.3);
        self.x.set_target(dock_x);
        self.y.set_target(dock_y);
    }

    /// Update all animations for one time step
    pub fn update(&mut self, dt: f32) {
        self.x.update(dt);
        self.y.update(dt);
        self.width.update(dt);
        self.height.update(dt);
        self.opacity.update(dt);
        self.scale.update(dt);
        self.corner_radius.update(dt);
    }

    /// Check if all animations have settled
    pub fn is_settled(&self) -> bool {
        self.x.settled && self.y.settled && self.width.settled &&
        self.height.settled && self.opacity.settled && self.scale.settled
    }
}

/// The master animation engine that manages all active animations
pub struct AnimationEngine {
    /// Per-window animations
    window_animations: HashMap<WindowId, WindowAnimation>,
    /// Workspace transition progress (0.0 = current, 1.0 = target)
    pub workspace_transition: Option<SpringAnimation>,
    /// Overview mode zoom level
    pub overview_zoom: SpringAnimation,
}

impl AnimationEngine {
    pub fn new() -> Self {
        Self {
            window_animations: HashMap::new(),
            workspace_transition: None,
            overview_zoom: SpringAnimation::gentle(1.0, 1.0),
        }
    }

    /// Register a new window animation
    pub fn add_window(&mut self, id: WindowId, x: f32, y: f32, w: f32, h: f32) {
        let mut anim = WindowAnimation::new(x, y, w, h);
        anim.play_open();
        self.window_animations.insert(id, anim);
    }

    /// Remove a window animation
    pub fn remove_window(&mut self, id: WindowId) {
        self.window_animations.remove(&id);
    }

    /// Get a window's current animation state
    pub fn get_window_animation(&self, id: WindowId) -> Option<&WindowAnimation> {
        self.window_animations.get(&id)
    }

    /// Get a mutable reference to a window's animation
    pub fn get_window_animation_mut(&mut self, id: WindowId) -> Option<&mut WindowAnimation> {
        self.window_animations.get_mut(&id)
    }

    /// Start workspace switch animation
    pub fn start_workspace_switch(&mut self, direction: f32) {
        let mut anim = SpringAnimation::gentle(0.0, direction);
        self.workspace_transition = Some(anim);
    }

    /// Enter overview mode
    pub fn enter_overview(&mut self) {
        self.overview_zoom.set_target(0.7);
    }

    /// Exit overview mode
    pub fn exit_overview(&mut self) {
        self.overview_zoom.set_target(1.0);
    }

    /// Update all active animations
    pub fn update(&mut self, dt: f32) {
        // Update window animations
        for anim in self.window_animations.values_mut() {
            anim.update(dt);
        }

        // Update workspace transition
        if let Some(ref mut ws) = self.workspace_transition {
            ws.update(dt);
            if ws.settled {
                self.workspace_transition = None;
            }
        }

        // Update overview zoom
        self.overview_zoom.update(dt);
    }

    /// Check if any animations are still running
    pub fn has_active_animations(&self) -> bool {
        self.window_animations.values().any(|a| !a.is_settled()) ||
        self.workspace_transition.is_some() ||
        !self.overview_zoom.settled
    }
}
