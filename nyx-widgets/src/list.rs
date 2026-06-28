//! # Nyx List Widget

use iced::widget::{scrollable, column, button, text, Row};
use iced::{Element, Length, Border, Background, Color, Theme, Padding, Alignment};
use iced::widget::button::{Status, Style as ButtonStyle};
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: String, pub primary_text: String,
    pub secondary_text: Option<String>, pub icon: Option<String>,
}

pub fn nyx_list<'a, Message: Clone + 'a>(
    items: &[ListItem], selected_id: Option<&str>,
    on_select: impl Fn(String) -> Message + 'a + Clone, theme: &NyxTheme,
) -> Element<'a, Message> {
    let mut col = column![].spacing(Spacing::XXXS);
    for item in items {
        let is_selected = selected_id.map_or(false, |id| id == item.id);
        let item_theme = theme.clone();
        let item_id = item.id.clone();
        let on_select_clone = on_select.clone();

        let mut row = Row::new().spacing(Spacing::SM).align_y(Alignment::Center);
        if let Some(ref icon) = item.icon {
            row = row.push(text(icon.clone()).size(Typography::SIZE_BODY_LG));
        }
        let mut text_col = iced::widget::column![];
        text_col = text_col.push(text(item.primary_text.clone()).size(Typography::SIZE_BODY));
        if let Some(ref secondary) = item.secondary_text {
            text_col = text_col.push(text(secondary.clone()).size(Typography::SIZE_CAPTION).color(item_theme.colors.text_tertiary));
        }
        row = row.push(text_col);

        let btn = button(row).width(Length::Fill)
            .padding(Padding::from([Spacing::XS, Spacing::SM]))
            .on_press(on_select_clone(item_id))
            .style(move |_t: &Theme, status: Status| list_item_style(&item_theme, is_selected, status));
        col = col.push(btn);
    }
    scrollable(col).height(Length::Fill).into()
}

fn list_item_style(theme: &NyxTheme, is_selected: bool, status: Status) -> ButtonStyle {
    let colors = &theme.colors;
    if is_selected {
        return ButtonStyle {
            background: Some(Background::Color(colors.accent_subtle)), text_color: colors.accent,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
            shadow: iced::Shadow::default(), snap: false,
        };
    }
    let (bg, tc) = match status {
        Status::Active | Status::Disabled => (None, colors.text_primary),
        Status::Hovered => (Some(Background::Color(Color::from_rgba(colors.text_primary.r, colors.text_primary.g, colors.text_primary.b, 0.05))), colors.text_primary),
        Status::Pressed => (Some(Background::Color(Color::from_rgba(colors.text_primary.r, colors.text_primary.g, colors.text_primary.b, 0.08))), colors.text_primary),
    };
    ButtonStyle {
        background: bg, text_color: tc,
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
        shadow: iced::Shadow::default(), snap: false,
    }
}
