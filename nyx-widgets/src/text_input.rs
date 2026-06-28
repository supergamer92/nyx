//! # Nyx Text Input Widget
//!
//! Styled text input with variants: default, search, password.

use iced::widget::{text_input, Row, text};
use iced::{Element, Border, Background, Color, Theme};
use iced::widget::text_input::{Status, Style};
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

/// Text input visual variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    /// Standard text input
    Default,
    /// Search input with icon
    Search,
    /// Password with reveal toggle
    Password,
}

/// Create a styled Nyx text input
pub fn nyx_text_input<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> Message + 'a,
    variant: InputVariant,
    nyx_theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = nyx_theme.clone();

    let input = text_input(placeholder, value)
        .on_input(on_change)
        .padding(Spacing::XS)
        .size(Typography::SIZE_BODY)
        .style(move |_theme: &Theme, status: Status| {
            nyx_input_style(&theme_clone, status)
        });

    match variant {
        InputVariant::Default => input.into(),
        InputVariant::Search => {
            let theme_clone2 = nyx_theme.clone();
            Row::new()
                .spacing(Spacing::XS)
                .push(
                    text("🔍")
                        .size(Typography::SIZE_BODY)
                        .color(theme_clone2.colors.text_tertiary),
                )
                .push(input)
                .into()
        }
        InputVariant::Password => {
            input.secure(true).into()
        }
    }
}

/// Generate text input style based on state
fn nyx_input_style(theme: &NyxTheme, status: Status) -> Style {
    let colors = &theme.colors;

    match status {
        Status::Active => Style {
            background: Background::Color(colors.bg_surface),
            border: Border {
                color: colors.border,
                width: 1.0,
                radius: Radii::MD.into(),
            },
            icon: colors.text_tertiary,
            placeholder: colors.text_tertiary,
            value: colors.text_primary,
            selection: colors.accent_subtle,
        },
        Status::Hovered => Style {
            background: Background::Color(colors.bg_surface),
            border: Border {
                color: colors.text_tertiary,
                width: 1.0,
                radius: Radii::MD.into(),
            },
            icon: colors.text_secondary,
            placeholder: colors.text_tertiary,
            value: colors.text_primary,
            selection: colors.accent_subtle,
        },
        Status::Focused { .. } => Style {
            background: Background::Color(colors.bg_surface),
            border: Border {
                color: colors.border_focused,
                width: 2.0,
                radius: Radii::MD.into(),
            },
            icon: colors.accent,
            placeholder: colors.text_tertiary,
            value: colors.text_primary,
            selection: colors.accent_subtle,
        },
        Status::Disabled => Style {
            background: Background::Color(Color::from_rgba(
                colors.bg_surface.r,
                colors.bg_surface.g,
                colors.bg_surface.b,
                0.5,
            )),
            border: Border {
                color: Color::from_rgba(
                    colors.border.r,
                    colors.border.g,
                    colors.border.b,
                    0.5,
                ),
                width: 1.0,
                radius: Radii::MD.into(),
            },
            icon: colors.text_disabled,
            placeholder: colors.text_disabled,
            value: colors.text_disabled,
            selection: Color::TRANSPARENT,
        },
    }
}
