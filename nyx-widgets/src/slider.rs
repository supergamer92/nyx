//! # Nyx Slider Widget
//!
//! A styled volume/brightness slider for the Nyx Lux design system.

use iced::widget::slider;
use iced::{Element, Theme, Border, Background};
use iced::widget::slider::{Status, Style};
use crate::theme::{NyxTheme, Radii};

/// Create a styled Nyx slider
pub fn nyx_slider<'a, Message: Clone + 'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();

    slider(range, value, on_change)
        .step(0.01)
        .style(move |_theme: &Theme, status: Status| {
            nyx_slider_style(&theme_clone, status)
        })
        .into()
}

fn nyx_slider_style(theme: &NyxTheme, status: Status) -> Style {
    let colors = &theme.colors;

    let (handle_color, _rail_active_color) = match status {
        Status::Active => (colors.accent, colors.accent),
        Status::Hovered => (colors.accent_hover, colors.accent_hover),
        Status::Dragged => (colors.accent_pressed, colors.accent_pressed),
    };

    Style {
        rail: iced::widget::slider::Rail {
            backgrounds: (handle_color.into(), colors.border.into()),
            width: 4.0,
            border: Border {
                radius: Radii::FULL.into(),
                ..Border::default()
            },
        },
        handle: iced::widget::slider::Handle {
            shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
            background: handle_color.into(),
            border_width: 2.0,
            border_color: iced::Color::WHITE,
        },
    }
}
