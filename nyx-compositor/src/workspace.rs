//! # Workspace Manager
//!
//! Manages virtual desktops (workspaces). Each workspace holds a set of windows
//! and maintains its own layout state.

use crate::window::WindowId;
use tracing::info;

/// A single workspace (virtual desktop)
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    /// Window IDs assigned to this workspace, in stacking order (back to front)
    pub windows: Vec<WindowId>,
    /// The focused window on this workspace
    pub focused: Option<WindowId>,
}

impl Workspace {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            name: format!("Workspace {}", id + 1),
            windows: Vec::new(),
            focused: None,
        }
    }

    /// Add a window to this workspace
    pub fn add_window(&mut self, id: WindowId) {
        if !self.windows.contains(&id) {
            self.windows.push(id);
            self.focused = Some(id);
        }
    }

    /// Remove a window from this workspace
    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.retain(|w| *w != id);
        if self.focused == Some(id) {
            self.focused = self.windows.last().copied();
        }
    }

    /// Raise a window to the top of the stacking order
    pub fn raise_window(&mut self, id: WindowId) {
        self.windows.retain(|w| *w != id);
        self.windows.push(id);
        self.focused = Some(id);
    }

    /// Check if this workspace has any windows
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }
}

/// Manages all workspaces
pub struct WorkspaceManager {
    pub workspaces: Vec<Workspace>,
    pub active: usize,
    pub previous: usize,
}

impl WorkspaceManager {
    /// Create a new workspace manager with a given number of initial workspaces
    pub fn new(count: usize) -> Self {
        let workspaces = (0..count).map(Workspace::new).collect();
        info!("Created {} workspaces", count);
        Self {
            workspaces,
            active: 0,
            previous: 0,
        }
    }

    /// Get the active workspace
    pub fn active_workspace(&self) -> &Workspace {
        &self.workspaces[self.active]
    }

    /// Get the active workspace mutably
    pub fn active_workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active]
    }

    /// Switch to a workspace by index
    pub fn switch_to(&mut self, index: usize) {
        if index < self.workspaces.len() && index != self.active {
            self.previous = self.active;
            self.active = index;
            info!("Switched to workspace {} (\"{}\")",
                  index, self.workspaces[index].name);
        }
    }

    /// Switch to next workspace (wraps around)
    pub fn switch_next(&mut self) {
        let next = (self.active + 1) % self.workspaces.len();
        self.switch_to(next);
    }

    /// Switch to previous workspace (wraps around)
    pub fn switch_prev(&mut self) {
        let prev = if self.active == 0 {
            self.workspaces.len() - 1
        } else {
            self.active - 1
        };
        self.switch_to(prev);
    }

    /// Switch back to the last active workspace
    pub fn switch_back(&mut self) {
        let prev = self.previous;
        self.switch_to(prev);
    }

    /// Add a window to the active workspace
    pub fn add_window(&mut self, id: WindowId) {
        self.workspaces[self.active].add_window(id);
    }

    /// Move a window from its current workspace to a target workspace
    pub fn move_window_to(&mut self, window_id: WindowId, target: usize) {
        if target >= self.workspaces.len() {
            return;
        }
        // Remove from all workspaces
        for ws in &mut self.workspaces {
            ws.remove_window(window_id);
        }
        // Add to target
        self.workspaces[target].add_window(window_id);
        info!("Moved window {} to workspace {}", window_id, target);
    }

    /// Remove a window from all workspaces
    pub fn remove_window(&mut self, id: WindowId) {
        for ws in &mut self.workspaces {
            ws.remove_window(id);
        }
    }

    /// Get the workspace count
    pub fn count(&self) -> usize {
        self.workspaces.len()
    }

    /// Add a new workspace
    pub fn add_workspace(&mut self) {
        let id = self.workspaces.len();
        self.workspaces.push(Workspace::new(id));
        info!("Added workspace {} (total: {})", id, self.workspaces.len());
    }

    /// Remove the last workspace (if more than 1 remain)
    pub fn remove_last_workspace(&mut self) {
        if self.workspaces.len() > 1 {
            let removed = self.workspaces.pop().unwrap();
            // Move any windows from removed workspace to the last remaining
            let last = self.workspaces.len() - 1;
            for wid in removed.windows {
                self.workspaces[last].add_window(wid);
            }
            if self.active >= self.workspaces.len() {
                self.active = self.workspaces.len() - 1;
            }
        }
    }

    /// Find which workspace contains a window
    pub fn find_window(&self, id: WindowId) -> Option<usize> {
        self.workspaces.iter().position(|ws| ws.windows.contains(&id))
    }
}
