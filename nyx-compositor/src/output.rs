//! # Output Manager
//!
//! Manages physical displays — resolution, scaling, refresh rate, HDR, arrangement.

use tracing::info;

/// A physical display output
#[derive(Debug, Clone)]
pub struct Output {
    pub id: u64,
    pub name: String,
    pub make: String,
    pub model: String,

    // Physical properties
    pub width_mm: u32,
    pub height_mm: u32,

    // Current mode
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32, // Hz

    // Position in the virtual display space
    pub x: i32,
    pub y: i32,

    // Scaling
    pub scale: f32, // 1.0, 1.25, 1.5, 2.0, etc.

    // Features
    pub hdr: bool,
    pub vrr: bool, // Variable refresh rate (FreeSync/G-Sync)
    pub vrr_enabled: bool,

    // Transform
    pub transform: Transform,
}

/// Display transform (rotation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    Flipped,
    FlippedRotate90,
    FlippedRotate180,
    FlippedRotate270,
}

/// Manages all outputs
pub struct OutputManager {
    outputs: Vec<Output>,
    primary: Option<u64>,
    next_id: u64,
}

impl OutputManager {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
            primary: None,
            next_id: 1,
        }
    }

    /// Add a new output (hot-plug)
    pub fn add_output(&mut self, name: String, make: String, model: String,
                       width: u32, height: u32, refresh_rate: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        // Auto-detect optimal scale based on DPI
        let scale = self.auto_detect_scale(width, height, 0, 0);

        let output = Output {
            id,
            name: name.clone(),
            make,
            model,
            width_mm: 0,
            height_mm: 0,
            width,
            height,
            refresh_rate,
            x: self.next_output_x(),
            y: 0,
            scale,
            hdr: false,
            vrr: false,
            vrr_enabled: false,
            transform: Transform::Normal,
        };

        info!("Output added: {} ({}x{}@{:.0}Hz, scale {:.2}x)",
              name, width, height, refresh_rate, scale);

        // First output becomes primary
        if self.primary.is_none() {
            self.primary = Some(id);
        }

        self.outputs.push(output);
        id
    }

    /// Remove an output (hot-unplug)
    pub fn remove_output(&mut self, id: u64) {
        self.outputs.retain(|o| o.id != id);
        if self.primary == Some(id) {
            self.primary = self.outputs.first().map(|o| o.id);
        }
        info!("Output removed: {}", id);
    }

    /// Get all outputs
    pub fn all(&self) -> &[Output] {
        &self.outputs
    }

    /// Get primary output
    pub fn primary(&self) -> Option<&Output> {
        self.primary.and_then(|id| self.outputs.iter().find(|o| o.id == id))
    }

    /// Set output scale
    pub fn set_scale(&mut self, id: u64, scale: f32) {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.id == id) {
            output.scale = scale;
            info!("Output {} scale set to {:.2}x", output.name, scale);
        }
    }

    /// Auto-detect appropriate scale factor based on display DPI
    fn auto_detect_scale(&self, width: u32, height: u32,
                          width_mm: u32, height_mm: u32) -> f32 {
        if width_mm == 0 || height_mm == 0 {
            // No physical size info — use resolution heuristics
            if width >= 3840 { return 2.0; }    // 4K
            if width >= 2560 { return 1.5; }    // QHD
            return 1.0;                          // FHD or lower
        }

        // Calculate DPI
        let dpi_x = (width as f32) / (width_mm as f32 / 25.4);
        let dpi_y = (height as f32) / (height_mm as f32 / 25.4);
        let dpi = (dpi_x + dpi_y) / 2.0;

        // Map DPI to scale factor
        if dpi >= 192.0 { 2.0 }
        else if dpi >= 144.0 { 1.5 }
        else if dpi >= 120.0 { 1.25 }
        else { 1.0 }
    }

    /// Calculate X position for next output (place to the right)
    fn next_output_x(&self) -> i32 {
        self.outputs.iter()
            .map(|o| o.x + (o.width as f32 / o.scale) as i32)
            .max()
            .unwrap_or(0)
    }

    /// Get total virtual display bounds
    pub fn total_bounds(&self) -> (i32, i32, u32, u32) {
        if self.outputs.is_empty() {
            return (0, 0, 1920, 1080);
        }

        let min_x = self.outputs.iter().map(|o| o.x).min().unwrap_or(0);
        let min_y = self.outputs.iter().map(|o| o.y).min().unwrap_or(0);
        let max_x = self.outputs.iter()
            .map(|o| o.x + (o.width as f32 / o.scale) as i32)
            .max().unwrap_or(1920);
        let max_y = self.outputs.iter()
            .map(|o| o.y + (o.height as f32 / o.scale) as i32)
            .max().unwrap_or(1080);

        (min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32)
    }
}
