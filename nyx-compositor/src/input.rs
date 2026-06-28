//! # Input Manager
//!
//! Handles keyboard, mouse, and touchpad input.
//! Includes gesture recognition for touchpad (3/4 finger swipes, pinch).

use tracing::info;

/// Manages all input state and gesture recognition
pub struct InputManager {
    /// Currently pressed modifier keys
    pub modifiers: Modifiers,
    /// Pending input events to process
    pending_events: Vec<InputEvent>,
    /// Active gesture state
    pub gesture: Option<GestureState>,
    /// Keyboard shortcuts registry
    pub shortcuts: Vec<Shortcut>,
}

impl InputManager {
    pub fn new() -> Self {
        let mut manager = Self {
            modifiers: Modifiers::default(),
            pending_events: Vec::new(),
            gesture: None,
            shortcuts: Vec::new(),
        };

        // Register default shortcuts
        manager.register_defaults();
        manager
    }

    /// Process all pending input events
    pub fn process_pending(&mut self) {
        let events: Vec<InputEvent> = self.pending_events.drain(..).collect();
        for event in events {
            self.handle_event(event);
        }
    }

    /// Queue an input event for processing
    pub fn queue_event(&mut self, event: InputEvent) {
        self.pending_events.push(event);
    }

    fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::KeyPress { key, .. } => {
                // Check for shortcut matches
                for shortcut in &self.shortcuts {
                    if shortcut.key == key && shortcut.modifiers == self.modifiers {
                        info!("Shortcut triggered: {}", shortcut.description);
                    }
                }
            }
            InputEvent::MouseMove { x, y } => {
                // Update cursor position
                let _ = (x, y);
            }
            InputEvent::TouchpadGesture { fingers, dx, dy, .. } => {
                self.handle_gesture(fingers, dx, dy);
            }
            _ => {}
        }
    }

    fn handle_gesture(&mut self, fingers: u32, dx: f32, dy: f32) {
        match fingers {
            3 => {
                // 3-finger horizontal swipe = switch workspace
                if dx.abs() > dy.abs() && dx.abs() > 20.0 {
                    if dx > 0.0 {
                        info!("Gesture: switch to next workspace");
                    } else {
                        info!("Gesture: switch to previous workspace");
                    }
                }
            }
            4 => {
                // 4-finger up = overview mode
                if dy < -30.0 {
                    info!("Gesture: enter overview mode");
                }
                // 4-finger down = show desktop
                if dy > 30.0 {
                    info!("Gesture: show desktop");
                }
            }
            _ => {}
        }
    }

    /// Register default keyboard shortcuts
    fn register_defaults(&mut self) {
        self.shortcuts.extend(vec![
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Space,
                description: "Open app launcher".into(),
                action: ShortcutAction::OpenLauncher,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Tab,
                description: "Switch window (Alt-Tab style)".into(),
                action: ShortcutAction::SwitchWindow,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::L,
                description: "Lock screen".into(),
                action: ShortcutAction::LockScreen,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, shift: true, ..Default::default() },
                key: Key::S,
                description: "Screenshot (region select)".into(),
                action: ShortcutAction::Screenshot,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::E,
                description: "Open file manager".into(),
                action: ShortcutAction::OpenApp("nyx-files".into()),
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::T,
                description: "Open terminal".into(),
                action: ShortcutAction::OpenApp("nyx-terminal".into()),
            },
            // Window management
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Left,
                description: "Tile window left".into(),
                action: ShortcutAction::TileLeft,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Right,
                description: "Tile window right".into(),
                action: ShortcutAction::TileRight,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Up,
                description: "Maximize window".into(),
                action: ShortcutAction::Maximize,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Down,
                description: "Restore/minimize window".into(),
                action: ShortcutAction::Minimize,
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Q,
                description: "Close window".into(),
                action: ShortcutAction::CloseWindow,
            },
            // Workspace switching
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Num1,
                description: "Switch to workspace 1".into(),
                action: ShortcutAction::SwitchWorkspace(0),
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Num2,
                description: "Switch to workspace 2".into(),
                action: ShortcutAction::SwitchWorkspace(1),
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Num3,
                description: "Switch to workspace 3".into(),
                action: ShortcutAction::SwitchWorkspace(2),
            },
            Shortcut {
                modifiers: Modifiers { super_key: true, ..Default::default() },
                key: Key::Num4,
                description: "Switch to workspace 4".into(),
                action: ShortcutAction::SwitchWorkspace(3),
            },
        ]);
    }
}

/// Modifier key state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

/// Keyboard key identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Space, Tab, Return, Escape, Backspace, Delete,
    Left, Right, Up, Down,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    PrintScreen, ScrollLock, Pause,
}

/// Input events from hardware
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress { key: Key, modifiers: Modifiers },
    KeyRelease { key: Key, modifiers: Modifiers },
    MouseMove { x: f64, y: f64 },
    MouseButton { button: u32, pressed: bool },
    MouseScroll { dx: f64, dy: f64 },
    TouchpadGesture { fingers: u32, dx: f32, dy: f32, scale: f32 },
}

/// What a shortcut does when triggered
#[derive(Debug, Clone)]
pub enum ShortcutAction {
    OpenLauncher,
    SwitchWindow,
    LockScreen,
    Screenshot,
    OpenApp(String),
    TileLeft,
    TileRight,
    Maximize,
    Minimize,
    CloseWindow,
    SwitchWorkspace(usize),
    MoveToWorkspace(usize),
}

/// A registered keyboard shortcut
#[derive(Debug, Clone)]
pub struct Shortcut {
    pub modifiers: Modifiers,
    pub key: Key,
    pub description: String,
    pub action: ShortcutAction,
}

/// Gesture tracking state
#[derive(Debug, Clone)]
pub struct GestureState {
    pub fingers: u32,
    pub start_x: f32,
    pub start_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub scale: f32,
}
