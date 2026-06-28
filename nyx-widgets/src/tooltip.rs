//! # Nyx Tooltip

use iced::widget::{tooltip, container, text};
use iced::{Element, Border, Background, Color, Theme};
use iced::widget::container::Style;
use iced::widget::tooltip::Position;
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

pub fn nyx_tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tip: &str,
    position: Position,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();
    let tip_content = container(
        text(tip.to_string()).size(Typography::SIZE_CAPTION)
    )
    .padding(Spacing::XS)
    .style(move |_theme: &Theme| Style {
        background: Some(Background::Color(theme_clone.colors.bg_elevated)),
        border: Border { color: theme_clone.colors.border, width: 1.0, radius: Radii::SM.into() },
        text_color: Some(theme_clone.colors.text_primary),
        shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.15), offset: iced::Vector::new(0.0, 4.0), blur_radius: 12.0 },
        snap: false,
    });

    tooltip(content, tip_content, position)
        .gap(4.0)
        .into()
}
