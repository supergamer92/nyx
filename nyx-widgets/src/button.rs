//! # Nyx Button Widget
//!
//! Styled button variants for the Nyx Lux design system.
//! Supports Primary, Secondary, Ghost, Danger, and Icon-only styles.

use iced::widget::{button, text, Row};
use iced::{Element, Length, Padding, Border, Background, Color, Theme};
use iced::widget::button::{Status, Style};
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

/// Button visual variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Filled accent background — primary actions
    Primary,
    /// Bordered, transparent background — secondary actions
    Secondary,
    /// No border, no background — tertiary/inline actions
    Ghost,
    /// Red — destructive actions
    Danger,
    /// Compact, icon-only
    Icon,
}

/// Button size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    pub fn padding(&self) -> Padding {
        match self {
            Self::Small => Padding::from([Spacing::XXS, Spacing::XS]),
            Self::Medium => Padding::from([Spacing::XS, Spacing::MD]),
            Self::Large => Padding::from([Spacing::SM, Spacing::LG]),
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            Self::Small => Typography::SIZE_BODY_SM,
            Self::Medium => Typography::SIZE_BODY,
            Self::Large => Typography::SIZE_BODY_LG,
        }
    }

    pub fn icon_size(&self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Medium => 20.0,
            Self::Large => 24.0,
        }
    }
}

/// Create a styled Nyx button
pub fn nyx_button<'a, Message: 'a + Clone>(
    label: &str,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();
    let content = text(label.to_string())
        .size(size.font_size());

    button(content)
        .padding(size.padding())
        .style(move |_theme: &Theme, status: Status| {
            nyx_button_style(&theme_clone, variant, status)
        })
        .into()
}

/// Create a button with an icon and label
pub fn nyx_button_with_icon<'a, Message: 'a + Clone>(
    icon_text: &str,
    label: &str,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();

    let row = Row::new()
        .spacing(Spacing::XS)
        .push(text(icon_text.to_string()).size(size.icon_size()))
        .push(text(label.to_string()).size(size.font_size()));

    button(row)
        .padding(size.padding())
        .style(move |_theme: &Theme, status: Status| {
            nyx_button_style(&theme_clone, variant, status)
        })
        .into()
}

/// Helper to create a ButtonStyle with snap: false
fn btn_style(bg: Option<Color>, text_color: Color, border_color: Color, border_width: f32) -> Style {
    Style {
        background: bg.map(Background::Color),
        text_color,
        border: Border {
            color: border_color,
            width: border_width,
            radius: Radii::MD.into(),
        },
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

/// Generate button style based on variant and interaction state
fn nyx_button_style(theme: &NyxTheme, variant: ButtonVariant, status: Status) -> Style {
    let colors = &theme.colors;

    match (variant, status) {
        // Primary
        (ButtonVariant::Primary, Status::Active) =>
            btn_style(Some(colors.accent), colors.text_on_accent, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Primary, Status::Hovered) =>
            btn_style(Some(colors.accent_hover), colors.text_on_accent, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Primary, Status::Pressed) =>
            btn_style(Some(colors.accent_pressed), colors.text_on_accent, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Primary, Status::Disabled) =>
            btn_style(Some(Color::from_rgba(colors.accent.r, colors.accent.g, colors.accent.b, 0.4)),
                      Color::from_rgba(1.0, 1.0, 1.0, 0.5), Color::TRANSPARENT, 0.0),

        // Secondary
        (ButtonVariant::Secondary, Status::Active) =>
            btn_style(None, colors.text_primary, colors.border, 1.0),
        (ButtonVariant::Secondary, Status::Hovered) =>
            btn_style(Some(colors.accent_subtle), colors.accent, colors.accent, 1.0),
        (ButtonVariant::Secondary, Status::Pressed) =>
            btn_style(Some(Color::from_rgba(colors.accent.r, colors.accent.g, colors.accent.b, 0.2)),
                      colors.accent, colors.accent_pressed, 1.0),
        (ButtonVariant::Secondary, Status::Disabled) =>
            btn_style(None, colors.text_disabled,
                      Color::from_rgba(colors.border.r, colors.border.g, colors.border.b, 0.5), 1.0),

        // Ghost
        (ButtonVariant::Ghost, Status::Active) =>
            btn_style(None, colors.text_secondary, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Ghost, Status::Hovered) =>
            btn_style(Some(colors.accent_subtle), colors.text_primary, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Ghost, Status::Pressed) =>
            btn_style(Some(Color::from_rgba(colors.accent.r, colors.accent.g, colors.accent.b, 0.2)),
                      colors.text_primary, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Ghost, Status::Disabled) =>
            btn_style(None, colors.text_disabled, Color::TRANSPARENT, 0.0),

        // Danger
        (ButtonVariant::Danger, Status::Active) =>
            btn_style(Some(colors.error), colors.text_on_accent, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Danger, Status::Hovered) =>
            btn_style(Some(crate::theme::lighten(colors.error, 0.1)), colors.text_on_accent, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Danger, Status::Pressed) =>
            btn_style(Some(crate::theme::darken(colors.error, 0.1)), colors.text_on_accent, Color::TRANSPARENT, 0.0),
        (ButtonVariant::Danger, Status::Disabled) =>
            btn_style(Some(Color::from_rgba(colors.error.r, colors.error.g, colors.error.b, 0.4)),
                      Color::from_rgba(1.0, 1.0, 1.0, 0.5), Color::TRANSPARENT, 0.0),

        // Icon — same as Ghost
        (ButtonVariant::Icon, status) => nyx_button_style(theme, ButtonVariant::Ghost, status),
    }
}
