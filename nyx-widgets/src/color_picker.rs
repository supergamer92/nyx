//! # Nyx Color Picker
//!
//! HSL color wheel with palette extraction support.
//! Placeholder — full implementation uses canvas for the wheel.

use iced::widget::{column, text, Row, container, slider};
use iced::{Element, Length, Color, Theme, Border, Background};
use iced::widget::container::Style;
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

/// Color picker state
#[derive(Debug, Clone)]
pub struct ColorPickerState {
    pub hue: f32,        // 0.0 - 360.0
    pub saturation: f32, // 0.0 - 1.0
    pub lightness: f32,  // 0.0 - 1.0
}

impl ColorPickerState {
    pub fn new() -> Self {
        Self {
            hue: 260.0,
            saturation: 0.7,
            lightness: 0.55,
        }
    }

    /// Convert current HSL to Iced Color
    pub fn to_color(&self) -> Color {
        hsl_to_rgb(self.hue, self.saturation, self.lightness)
    }

    /// Create from an Iced Color
    pub fn from_color(color: Color) -> Self {
        let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
        Self { hue: h, saturation: s, lightness: l }
    }
}

impl Default for ColorPickerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined accent color presets
pub fn accent_presets() -> Vec<(&'static str, Color)> {
    vec![
        ("Indigo", crate::theme::hex("#7c5cfc")),
        ("Blue", crate::theme::hex("#3b82f6")),
        ("Cyan", crate::theme::hex("#06b6d4")),
        ("Teal", crate::theme::hex("#14b8a6")),
        ("Green", crate::theme::hex("#22c55e")),
        ("Yellow", crate::theme::hex("#eab308")),
        ("Orange", crate::theme::hex("#f97316")),
        ("Red", crate::theme::hex("#ef4444")),
        ("Pink", crate::theme::hex("#ec4899")),
        ("Purple", crate::theme::hex("#a855f7")),
    ]
}

// ───────────────────────────────────────────────────────────
// HSL ↔ RGB conversion
// ───────────────────────────────────────────────────────────

/// Convert HSL to RGB (Iced Color)
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    if s == 0.0 {
        return Color::from_rgb(l, l, l);
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let h = h / 360.0;

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    Color::from_rgb(r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 { t += 1.0; }
    if t > 1.0 { t -= 1.0; }
    if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0 / 2.0 { return q; }
    if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    p
}

/// Convert RGB to HSL
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

    let h = if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    (h * 360.0, s, l)
}
