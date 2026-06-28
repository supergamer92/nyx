//! # Nyx Sidebar / Navigation Rail

use iced::widget::{column, container, button, text, Row};
use iced::{Element, Length, Padding, Border, Background, Color, Theme, Alignment};
use iced::widget::button::{Status, Style as ButtonStyle};
use iced::widget::container::Style;
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

#[derive(Debug, Clone)]
pub struct SidebarItem {
    pub icon: String,
    pub label: String,
    pub id: String,
}

pub fn nyx_sidebar<'a, Message: Clone + 'a>(
    items: &[SidebarItem],
    active_id: &str,
    on_select: impl Fn(String) -> Message + 'a + Clone,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let theme_clone = theme.clone();
    let mut col = column![].spacing(Spacing::XXS).padding(Spacing::XS);

    for item in items {
        let is_active = item.id == active_id;
        let item_theme = theme.clone();
        let item_id = item.id.clone();
        let on_select_clone = on_select.clone();

        let row = Row::new()
            .spacing(Spacing::SM)
            .align_y(Alignment::Center)
            .push(text(item.icon.clone()).size(Typography::SIZE_BODY_LG))
            .push(text(item.label.clone()).size(Typography::SIZE_BODY));

        let btn = button(row)
            .width(Length::Fill)
            .padding(Padding::from([Spacing::XS, Spacing::SM]))
            .on_press(on_select_clone(item_id))
            .style(move |_theme: &Theme, status: Status| {
                sidebar_item_style(&item_theme, is_active, status)
            });

        col = col.push(btn);
    }

    container(col)
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(move |_theme: &Theme| Style {
            background: Some(Background::Color(theme_clone.colors.bg_surface)),
            border: Border { color: theme_clone.colors.divider, width: 0.0, radius: 0.0.into() },
            text_color: None,
            shadow: iced::Shadow::default(),
            snap: false,
        })
        .into()
}

fn sidebar_item_style(theme: &NyxTheme, is_active: bool, status: Status) -> ButtonStyle {
    let colors = &theme.colors;
    if is_active {
        ButtonStyle {
            background: Some(Background::Color(colors.accent_subtle)),
            text_color: colors.accent,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    } else {
        let (bg, tc) = match status {
            Status::Active | Status::Disabled => (None, colors.text_secondary),
            Status::Hovered => (Some(Background::Color(Color::from_rgba(colors.text_primary.r, colors.text_primary.g, colors.text_primary.b, 0.06))), colors.text_primary),
            Status::Pressed => (Some(Background::Color(Color::from_rgba(colors.text_primary.r, colors.text_primary.g, colors.text_primary.b, 0.1))), colors.text_primary),
        };
        ButtonStyle {
            background: bg, text_color: tc,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    }
}
