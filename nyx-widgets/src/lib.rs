//! # Nyx Widgets
//!
//! The Nyx Lux design system — a custom Iced widget library that provides
//! a consistent, GPU-rendered visual language for all Nyx OS applications.
//!
//! ## Design Principles
//!
//! - **Consistent**: Every widget follows the same design tokens
//! - **Performant**: GPU-rendered via wgpu, spring-physics animations
//! - **Adaptive**: Wallpaper-based color extraction, dark/light auto-switch
//! - **Accessible**: Keyboard navigation, screen reader support, high contrast

pub mod theme;
pub mod button;
pub mod text_input;
pub mod toggle;
pub mod slider;
pub mod card;
pub mod modal;
pub mod tooltip;
pub mod sidebar;
pub mod tabs;
pub mod toast;
pub mod progress;
pub mod list;
pub mod context_menu;
pub mod color_picker;

// Re-export theme for convenience
pub use theme::{NyxTheme, ColorPalette, Spacing, Typography};
