//! # Nyx Modal / Dialog

use iced::widget::{column, container, text, Row, Space};
use iced::{Element, Length, Border, Background, Color, Theme, Alignment};
use iced::widget::container::Style;
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalKind { Default, Warning, Danger }

pub fn nyx_modal<'a, Message: 'a>(
    title: &str,
    body: impl Into<Element<'a, Message>>,
    actions: Vec<Element<'a, Message>>,
    kind: ModalKind,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let colors = theme.colors.clone();
    let title_color = match kind {
        ModalKind::Default => colors.text_primary,
        ModalKind::Warning => colors.warning,
        ModalKind::Danger => colors.error,
    };

    let mut action_row = Row::new()
        .spacing(Spacing::XS)
        .push(Space::new().width(Length::Fill));
    for action in actions {
        action_row = action_row.push(action);
    }

    let content = column![
        text(title.to_string()).size(Typography::SIZE_HEADING_SM).color(title_color),
        body.into(),
        action_row,
    ]
    .spacing(Spacing::MD)
    .padding(Spacing::LG);

    let theme_clone = theme.clone();
    container(content)
        .width(Length::Fixed(420.0))
        .style(move |_theme: &Theme| Style {
            background: Some(Background::Color(theme_clone.colors.bg_elevated)),
            border: Border { color: theme_clone.colors.border, width: 1.0, radius: Radii::XXL.into() },
            text_color: Some(theme_clone.colors.text_primary),
            shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.25), offset: iced::Vector::new(0.0, 16.0), blur_radius: 48.0 },
            snap: false,
        })
        .into()
}
