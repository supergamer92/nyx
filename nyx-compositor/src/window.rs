//! # Window Manager
//!
//! Manages individual window properties: geometry, state, decorations,
//! tiling, snap zones, and window rules.

use crate::animation::WindowAnimation;
use tracing::info;

/// Unique window identifier
pub type WindowId = u64;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    /// Normal floating/tiled window
    Normal,
    /// Maximized (fills workspace, no gaps)
    Maximized,
    /// Minimized to dock
    Minimized,
    /// Fullscreen (covers everything including panels)
    Fullscreen,
    /// Being moved by the user
    Moving,
    /// Being resized by the user
    Resizing,
}

/// Tiling position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilePosition {
    /// Not tiled — floating
    None,
    /// Left half
    Left,
    /// Right half
    Right,
    /// Top-left quarter
    TopLeft,
    /// Top-right quarter
    TopRight,
    /// Bottom-left quarter
    BottomLeft,
    /// Bottom-right quarter
    BottomRight,
    /// Top half
    Top,
    /// Bottom half
    Bottom,
}

/// A managed window
#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,

    // Geometry (logical coordinates)
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    // Minimum and maximum size constraints
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,

    // State
    pub state: WindowState,
    pub tile: TilePosition,
    pub opacity: f32,
    pub focused: bool,

    // Saved geometry for restore from maximize/tile
    saved_geometry: Option<(f32, f32, f32, f32)>,

    // Decorations
    pub server_side_decorations: bool,
    pub corner_radius: f32,
    pub shadow_size: f32,
}

impl Window {
    pub fn new(id: WindowId, app_id: String, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            id,
            title: String::new(),
            app_id,
            x, y,
            width: w,
            height: h,
            min_width: 200.0,
            min_height: 150.0,
            max_width: f32::MAX,
            max_height: f32::MAX,
            state: WindowState::Normal,
            tile: TilePosition::None,
            opacity: 1.0,
            focused: false,
            saved_geometry: None,
            server_side_decorations: true,
            corner_radius: 12.0,
            shadow_size: 24.0,
        }
    }

    /// Save current geometry for later restore
    fn save_geometry(&mut self) {
        self.saved_geometry = Some((self.x, self.y, self.width, self.height));
    }

    /// Restore previously saved geometry
    fn restore_geometry(&mut self) {
        if let Some((x, y, w, h)) = self.saved_geometry.take() {
            self.x = x;
            self.y = y;
            self.width = w;
            self.height = h;
        }
    }

    /// Maximize the window
    pub fn maximize(&mut self, screen_w: f32, screen_h: f32, top_bar_h: f32) {
        if self.state != WindowState::Maximized {
            self.save_geometry();
            self.state = WindowState::Maximized;
            self.x = 0.0;
            self.y = top_bar_h;
            self.width = screen_w;
            self.height = screen_h - top_bar_h;
            self.corner_radius = 0.0;
            self.shadow_size = 0.0;
        }
    }

    /// Unmaximize / restore the window
    pub fn unmaximize(&mut self) {
        if self.state == WindowState::Maximized {
            self.state = WindowState::Normal;
            self.restore_geometry();
            self.corner_radius = 12.0;
            self.shadow_size = 24.0;
        }
    }

    /// Toggle maximize state
    pub fn toggle_maximize(&mut self, screen_w: f32, screen_h: f32, top_bar_h: f32) {
        if self.state == WindowState::Maximized {
            self.unmaximize();
        } else {
            self.maximize(screen_w, screen_h, top_bar_h);
        }
    }

    /// Minimize to dock
    pub fn minimize(&mut self) {
        if self.state != WindowState::Minimized {
            self.save_geometry();
            self.state = WindowState::Minimized;
        }
    }

    /// Tile the window to a snap zone
    pub fn tile_to(&mut self, position: TilePosition, screen_w: f32, screen_h: f32,
                    top_bar_h: f32, gap: f32) {
        if self.tile == TilePosition::None {
            self.save_geometry();
        }
        self.tile = position;
        self.state = WindowState::Normal;
        self.corner_radius = 12.0;
        self.shadow_size = 16.0;

        let usable_h = screen_h - top_bar_h;
        let half_w = (screen_w - gap * 3.0) / 2.0;
        let half_h = (usable_h - gap * 3.0) / 2.0;

        match position {
            TilePosition::None => {
                self.restore_geometry();
            }
            TilePosition::Left => {
                self.x = gap;
                self.y = top_bar_h + gap;
                self.width = half_w;
                self.height = usable_h - gap * 2.0;
            }
            TilePosition::Right => {
                self.x = half_w + gap * 2.0;
                self.y = top_bar_h + gap;
                self.width = half_w;
                self.height = usable_h - gap * 2.0;
            }
            TilePosition::TopLeft => {
                self.x = gap;
                self.y = top_bar_h + gap;
                self.width = half_w;
                self.height = half_h;
            }
            TilePosition::TopRight => {
                self.x = half_w + gap * 2.0;
                self.y = top_bar_h + gap;
                self.width = half_w;
                self.height = half_h;
            }
            TilePosition::BottomLeft => {
                self.x = gap;
                self.y = top_bar_h + half_h + gap * 2.0;
                self.width = half_w;
                self.height = half_h;
            }
            TilePosition::BottomRight => {
                self.x = half_w + gap * 2.0;
                self.y = top_bar_h + half_h + gap * 2.0;
                self.width = half_w;
                self.height = half_h;
            }
            TilePosition::Top => {
                self.x = gap;
                self.y = top_bar_h + gap;
                self.width = screen_w - gap * 2.0;
                self.height = half_h;
            }
            TilePosition::Bottom => {
                self.x = gap;
                self.y = top_bar_h + half_h + gap * 2.0;
                self.width = screen_w - gap * 2.0;
                self.height = half_h;
            }
        }
    }

    /// Apply animation values to the window
    pub fn apply_animation(&mut self, anim: &WindowAnimation) {
        // Animation values are used for rendering only —
        // actual geometry stays fixed for input hit-testing
    }
}

/// Manages all windows
pub struct WindowManager {
    windows: Vec<Window>,
    next_id: WindowId,
    /// Gap between tiled windows (pixels)
    pub tile_gap: f32,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            tile_gap: 8.0,
        }
    }

    /// Create and register a new window
    pub fn create_window(&mut self, app_id: String, x: f32, y: f32,
                          w: f32, h: f32) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;

        let window = Window::new(id, app_id.clone(), x, y, w, h);
        info!("Window created: id={}, app={}, {}x{}", id, app_id, w, h);
        self.windows.push(window);
        id
    }

    /// Remove a window
    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.retain(|w| w.id != id);
        info!("Window removed: id={}", id);
    }

    /// Get a window by ID
    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }

    /// Get a mutable window by ID
    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    /// Get all windows
    pub fn all(&self) -> &[Window] {
        &self.windows
    }

    /// Get all windows mutably
    pub fn all_mut(&mut self) -> &mut [Window] {
        &mut self.windows
    }

    /// Get window count
    pub fn count(&self) -> usize {
        self.windows.len()
    }

    /// Find the window at a given point (for click handling)
    pub fn window_at(&self, x: f32, y: f32, workspace_windows: &[WindowId]) -> Option<WindowId> {
        // Iterate in reverse stacking order (front to back)
        for id in workspace_windows.iter().rev() {
            if let Some(window) = self.get(*id) {
                if window.state != WindowState::Minimized
                    && x >= window.x
                    && x <= window.x + window.width
                    && y >= window.y
                    && y <= window.y + window.height
                {
                    return Some(*id);
                }
            }
        }
        None
    }

    /// Focus a window (unfocuses all others)
    pub fn focus(&mut self, id: WindowId) {
        for window in &mut self.windows {
            window.focused = window.id == id;
        }
    }

    /// Get the currently focused window
    pub fn focused(&self) -> Option<&Window> {
        self.windows.iter().find(|w| w.focused)
    }
}
