//! # Nyx Lux Theme
//!
//! The complete design token system for Nyx OS. All colors, spacing,
//! typography, radii, and shadows are defined here.

use iced::Color;

// ───────────────────────────────────────────────────────────
// Color Palette
// ───────────────────────────────────────────────────────────

/// Complete color palette for Nyx Lux design system.
/// Colors are defined in HSL-friendly values for easy theming.
#[derive(Debug, Clone)]
pub struct ColorPalette {
    // Backgrounds
    pub bg_base: Color,
    pub bg_surface: Color,
    pub bg_elevated: Color,
    pub bg_overlay: Color,

    // Text
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
    pub text_disabled: Color,
    pub text_on_accent: Color,

    // Accent (user-selectable or wallpaper-adaptive)
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_pressed: Color,
    pub accent_subtle: Color,

    // Semantic
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Borders & Dividers
    pub border: Color,
    pub border_focused: Color,
    pub divider: Color,

    // Dock & Panels
    pub panel_bg: Color,
    pub dock_bg: Color,
}

impl ColorPalette {
    /// Dark theme — true dark with OLED-friendly deep blacks
    pub fn dark() -> Self {
        Self {
            // Backgrounds — deep, warm-tinted blacks
            bg_base: hex("#0a0a0c"),
            bg_surface: hex("#141418"),
            bg_elevated: hex("#1c1c22"),
            bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.6),

            // Text — crisp whites with opacity hierarchy
            text_primary: hex("#f0f0f3"),
            text_secondary: hex("#a0a0ab"),
            text_tertiary: hex("#6b6b78"),
            text_disabled: hex("#45454f"),
            text_on_accent: hex("#ffffff"),

            // Accent — vibrant indigo-purple
            accent: hex("#7c5cfc"),
            accent_hover: hex("#8e72ff"),
            accent_pressed: hex("#6a48e0"),
            accent_subtle: Color::from_rgba(0.486, 0.361, 0.988, 0.15),

            // Semantic
            success: hex("#34d399"),
            warning: hex("#fbbf24"),
            error: hex("#f87171"),
            info: hex("#60a5fa"),

            // Borders
            border: hex("#2a2a32"),
            border_focused: hex("#7c5cfc"),
            divider: hex("#1e1e26"),

            // Panels — semi-transparent for blur effect
            panel_bg: Color::from_rgba(0.08, 0.08, 0.1, 0.85),
            dock_bg: Color::from_rgba(0.1, 0.1, 0.12, 0.8),
        }
    }

    /// Light theme — warm whites, not clinical
    pub fn light() -> Self {
        Self {
            // Backgrounds — warm off-whites
            bg_base: hex("#fafaf9"),
            bg_surface: hex("#ffffff"),
            bg_elevated: hex("#ffffff"),
            bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.3),

            // Text — warm blacks
            text_primary: hex("#1a1a1e"),
            text_secondary: hex("#5a5a66"),
            text_tertiary: hex("#8a8a96"),
            text_disabled: hex("#c0c0c8"),
            text_on_accent: hex("#ffffff"),

            // Accent — same base, slightly adjusted for light
            accent: hex("#6b4ee0"),
            accent_hover: hex("#7c5cfc"),
            accent_pressed: hex("#5a3cc8"),
            accent_subtle: Color::from_rgba(0.42, 0.306, 0.878, 0.1),

            // Semantic
            success: hex("#16a34a"),
            warning: hex("#d97706"),
            error: hex("#dc2626"),
            info: hex("#2563eb"),

            // Borders
            border: hex("#e4e4e7"),
            border_focused: hex("#6b4ee0"),
            divider: hex("#f0f0f2"),

            // Panels
            panel_bg: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
            dock_bg: Color::from_rgba(0.96, 0.96, 0.97, 0.85),
        }
    }
}

// ───────────────────────────────────────────────────────────
// Spacing
// ───────────────────────────────────────────────────────────

/// Spacing scale based on 4px grid
#[derive(Debug, Clone, Copy)]
pub struct Spacing;

impl Spacing {
    pub const XXXS: f32 = 2.0;
    pub const XXS: f32 = 4.0;
    pub const XS: f32 = 8.0;
    pub const SM: f32 = 12.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 24.0;
    pub const XL: f32 = 32.0;
    pub const XXL: f32 = 48.0;
    pub const XXXL: f32 = 64.0;
}

// ───────────────────────────────────────────────────────────
// Typography
// ───────────────────────────────────────────────────────────

/// Typography scale for Nyx Lux
#[derive(Debug, Clone, Copy)]
pub struct Typography;

impl Typography {
    // Font families (bundled with Nyx)
    pub const FONT_UI: &str = "Inter";
    pub const FONT_MONO: &str = "JetBrains Mono";

    // Size scale
    pub const SIZE_CAPTION: f32 = 11.0;
    pub const SIZE_BODY_SM: f32 = 13.0;
    pub const SIZE_BODY: f32 = 14.0;
    pub const SIZE_BODY_LG: f32 = 16.0;
    pub const SIZE_HEADING_SM: f32 = 18.0;
    pub const SIZE_HEADING: f32 = 22.0;
    pub const SIZE_HEADING_LG: f32 = 28.0;
    pub const SIZE_DISPLAY: f32 = 36.0;
    pub const SIZE_DISPLAY_LG: f32 = 48.0;

    // Line heights
    pub const LINE_HEIGHT_TIGHT: f32 = 1.2;
    pub const LINE_HEIGHT_NORMAL: f32 = 1.5;
    pub const LINE_HEIGHT_RELAXED: f32 = 1.75;

    // Weights (for reference — actual weight applied via font loading)
    pub const WEIGHT_REGULAR: u16 = 400;
    pub const WEIGHT_MEDIUM: u16 = 500;
    pub const WEIGHT_SEMIBOLD: u16 = 600;
    pub const WEIGHT_BOLD: u16 = 700;
}

// ───────────────────────────────────────────────────────────
// Border Radius
// ───────────────────────────────────────────────────────────

/// Border radius scale
#[derive(Debug, Clone, Copy)]
pub struct Radii;

impl Radii {
    pub const NONE: f32 = 0.0;
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 12.0;
    pub const XL: f32 = 16.0;
    pub const XXL: f32 = 20.0;
    pub const FULL: f32 = 9999.0;
}

// ───────────────────────────────────────────────────────────
// Shadows
// ───────────────────────────────────────────────────────────

/// Shadow definitions (applied via custom rendering)
#[derive(Debug, Clone)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub color: Color,
}

impl Shadow {
    pub fn sm() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 1.0,
            blur_radius: 3.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
        }
    }

    pub fn md() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 4.0,
            blur_radius: 12.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
        }
    }

    pub fn lg() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 8.0,
            blur_radius: 32.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
        }
    }

    pub fn xl() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 16.0,
            blur_radius: 48.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
        }
    }
}

// ───────────────────────────────────────────────────────────
// Animation Curves
// ───────────────────────────────────────────────────────────

/// Spring animation parameters for natural motion
#[derive(Debug, Clone, Copy)]
pub struct SpringConfig {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

impl SpringConfig {
    /// Snappy — for toggles, buttons
    pub fn snappy() -> Self {
        Self { stiffness: 400.0, damping: 30.0, mass: 1.0 }
    }

    /// Default — for most UI transitions
    pub fn default_spring() -> Self {
        Self { stiffness: 300.0, damping: 25.0, mass: 1.0 }
    }

    /// Gentle — for large transitions (workspace switch, overview)
    pub fn gentle() -> Self {
        Self { stiffness: 200.0, damping: 22.0, mass: 1.0 }
    }

    /// Bouncy — for playful interactions (dock magnification)
    pub fn bouncy() -> Self {
        Self { stiffness: 350.0, damping: 18.0, mass: 1.0 }
    }
}

// ───────────────────────────────────────────────────────────
// Main Theme
// ───────────────────────────────────────────────────────────

/// The Nyx OS theme mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

/// The master Nyx theme that all widgets reference
#[derive(Debug, Clone)]
pub struct NyxTheme {
    pub mode: ThemeMode,
    pub colors: ColorPalette,
    pub accent_override: Option<Color>,
}

impl NyxTheme {
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ColorPalette::dark(),
            accent_override: None,
        }
    }

    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ColorPalette::light(),
            accent_override: None,
        }
    }

    /// Override the accent color (from wallpaper extraction or user choice)
    pub fn with_accent(mut self, color: Color) -> Self {
        self.accent_override = Some(color);
        self.colors.accent = color;
        // Derive hover/pressed variants
        self.colors.accent_hover = lighten(color, 0.1);
        self.colors.accent_pressed = darken(color, 0.1);
        self.colors.accent_subtle = Color::from_rgba(color.r, color.g, color.b, 0.15);
        self.colors.border_focused = color;
        self
    }

    /// Get the current accent color
    pub fn accent(&self) -> Color {
        self.accent_override.unwrap_or(self.colors.accent)
    }

    /// Check if dark mode is active
    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }
}

impl Default for NyxTheme {
    fn default() -> Self {
        Self::dark()
    }
}

// ───────────────────────────────────────────────────────────
// Color Utilities
// ───────────────────────────────────────────────────────────

/// Parse a hex color string like "#ff00aa" into an Iced Color
pub fn hex(s: &str) -> Color {
    let s = s.trim_start_matches('#');
    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
    Color::from_rgb8(r, g, b)
}

/// Lighten a color by a factor (0.0 - 1.0)
pub fn lighten(color: Color, amount: f32) -> Color {
    Color::from_rgba(
        (color.r + (1.0 - color.r) * amount).min(1.0),
        (color.g + (1.0 - color.g) * amount).min(1.0),
        (color.b + (1.0 - color.b) * amount).min(1.0),
        color.a,
    )
}

/// Darken a color by a factor (0.0 - 1.0)
pub fn darken(color: Color, amount: f32) -> Color {
    Color::from_rgba(
        (color.r * (1.0 - amount)).max(0.0),
        (color.g * (1.0 - amount)).max(0.0),
        (color.b * (1.0 - amount)).max(0.0),
        color.a,
    )
}

/// Mix two colors together with a ratio (0.0 = all a, 1.0 = all b)
pub fn mix(a: Color, b: Color, ratio: f32) -> Color {
    let ratio = ratio.clamp(0.0, 1.0);
    Color::from_rgba(
        a.r + (b.r - a.r) * ratio,
        a.g + (b.g - a.g) * ratio,
        a.b + (b.b - a.b) * ratio,
        a.a + (b.a - a.a) * ratio,
    )
}

/// Add alpha transparency to an existing color
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::from_rgba(color.r, color.g, color.b, alpha)
}
