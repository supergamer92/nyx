//! # Nyx Card Widget
//!
//! A content card with subtle shadow, rounded corners, and hover lift effect.

use iced::widget::container;
use iced::{Element, Length, Padding, Border, Background, Color, Theme};
use iced::widget::container::Style;
use crate::theme::{NyxTheme, Radii, Spacing};

/// Card visual variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardVariant {
    Surface,
    Elevated,
    Outlined,
    Interactive,
}

/// Helper to create container style with snap: false
fn card_style(bg: Color, border_color: Color, border_width: f32, radius: f32, shadow: iced::Shadow, text_color: Color) -> Style {
    Style {
        background: Some(Background::Color(bg)),
        border: Border { color: border_color, width: border_width, radius: radius.into() },
        text_color: Some(text_color),
        shadow,
        snap: false,
    }
}

/// Create a styled Nyx card
pub fn nyx_card<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    variant: CardVariant,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();

    container(content)
        .padding(Spacing::MD)
        .width(Length::Fill)
        .style(move |_theme: &Theme| nyx_card_style(&theme_clone, variant))
        .into()
}

/// Create a card with custom padding
pub fn nyx_card_padded<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    padding: Padding,
    variant: CardVariant,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();

    container(content)
        .padding(padding)
        .width(Length::Fill)
        .style(move |_theme: &Theme| nyx_card_style(&theme_clone, variant))
        .into()
}

fn nyx_card_style(theme: &NyxTheme, variant: CardVariant) -> Style {
    let colors = &theme.colors;

    match variant {
        CardVariant::Surface => card_style(
            colors.bg_surface, Color::TRANSPARENT, 0.0, Radii::LG,
            iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.06), offset: iced::Vector::new(0.0, 2.0), blur_radius: 8.0 },
            colors.text_primary,
        ),
        CardVariant::Elevated => card_style(
            colors.bg_elevated, Color::TRANSPARENT, 0.0, Radii::LG,
            iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.12), offset: iced::Vector::new(0.0, 4.0), blur_radius: 16.0 },
            colors.text_primary,
        ),
        CardVariant::Outlined => card_style(
            colors.bg_surface, colors.border, 1.0, Radii::LG,
            iced::Shadow::default(),
            colors.text_primary,
        ),
        CardVariant::Interactive => card_style(
            colors.bg_surface, colors.border, 1.0, Radii::LG,
            iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.08), offset: iced::Vector::new(0.0, 2.0), blur_radius: 8.0 },
            colors.text_primary,
        ),
    }
}
