//! # Nyx Tabs Widget

use iced::widget::{button, text, Row, container};
use iced::{Element, Border, Background, Color, Theme, Padding};
use iced::widget::button::{Status, Style as ButtonStyle};
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

#[derive(Debug, Clone)]
pub struct Tab { pub label: String, pub id: String }

pub fn nyx_tabs<'a, Message: Clone + 'a>(
    tabs: &[Tab], active_id: &str,
    on_select: impl Fn(String) -> Message + 'a + Clone,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let mut row = Row::new().spacing(Spacing::XXS);
    for tab in tabs {
        let is_active = tab.id == active_id;
        let tab_theme = theme.clone();
        let tab_id = tab.id.clone();
        let on_select_clone = on_select.clone();
        let btn = button(text(tab.label.clone()).size(Typography::SIZE_BODY))
            .padding(Padding::from([Spacing::XS, Spacing::MD]))
            .on_press(on_select_clone(tab_id))
            .style(move |_t: &Theme, status: Status| tab_style(&tab_theme, is_active, status));
        row = row.push(btn);
    }
    let tc = theme.clone();
    container(row)
        .padding(Spacing::XXS)
        .style(move |_t: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(tc.colors.bg_surface)),
            border: Border { color: tc.colors.border, width: 1.0, radius: Radii::LG.into() },
            text_color: None, shadow: iced::Shadow::default(), snap: false,
        })
        .into()
}

fn tab_style(theme: &NyxTheme, is_active: bool, status: Status) -> ButtonStyle {
    let colors = &theme.colors;
    if is_active {
        ButtonStyle {
            background: Some(Background::Color(colors.accent)), text_color: colors.text_on_accent,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
            shadow: iced::Shadow::default(), snap: false,
        }
    } else {
        let (bg, tc) = match status {
            Status::Active | Status::Disabled => (None, colors.text_secondary),
            Status::Hovered => (Some(Background::Color(colors.accent_subtle)), colors.text_primary),
            Status::Pressed => (Some(Background::Color(Color::from_rgba(colors.accent.r, colors.accent.g, colors.accent.b, 0.3))), colors.text_primary),
        };
        ButtonStyle {
            background: bg, text_color: tc,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
            shadow: iced::Shadow::default(), snap: false,
        }
    }
}
