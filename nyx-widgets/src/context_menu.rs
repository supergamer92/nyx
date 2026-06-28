//! # Nyx Context Menu

use iced::widget::{column, button, container, text, Row, Space};
use iced::{Element, Length, Border, Background, Color, Theme, Padding, Alignment};
use iced::widget::button::{Status, Style as ButtonStyle};
use iced::widget::container::Style;
use crate::theme::{NyxTheme, Radii, Spacing, Typography};

#[derive(Debug, Clone)]
pub enum ContextMenuItem {
    Action { label: String, icon: Option<String>, shortcut: Option<String>, id: String },
    Separator,
}

pub fn nyx_context_menu<'a, Message: Clone + 'a>(
    items: &[ContextMenuItem],
    on_action: impl Fn(String) -> Message + 'a + Clone,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let tc = theme.clone();
    let mut col = column![].spacing(Spacing::XXXS).padding(Spacing::XXS);

    for item in items {
        match item {
            ContextMenuItem::Action { label, icon, shortcut, id } => {
                let item_theme = theme.clone();
                let item_id = id.clone();
                let on_action_clone = on_action.clone();

                let mut row = Row::new().spacing(Spacing::SM).align_y(Alignment::Center);
                if let Some(ref icon_str) = icon {
                    row = row.push(text(icon_str.clone()).size(Typography::SIZE_BODY));
                } else {
                    row = row.push(Space::new().width(Length::Fixed(Typography::SIZE_BODY)));
                }
                row = row.push(text(label.clone()).size(Typography::SIZE_BODY));
                row = row.push(Space::new().width(Length::Fill));
                if let Some(ref sc) = shortcut {
                    row = row.push(text(sc.clone()).size(Typography::SIZE_CAPTION).color(item_theme.colors.text_tertiary));
                }

                let btn = button(row).width(Length::Fill)
                    .padding(Padding::from([Spacing::XXS, Spacing::XS]))
                    .on_press(on_action_clone(item_id))
                    .style(move |_t: &Theme, status: Status| ctx_style(&item_theme, status));
                col = col.push(btn);
            }
            ContextMenuItem::Separator => {
                let sep_theme = theme.clone();
                col = col.push(
                    container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
                        .width(Length::Fill)
                        .style(move |_t: &Theme| Style {
                            background: Some(Background::Color(sep_theme.colors.divider)),
                            border: Border::default(), text_color: None,
                            shadow: iced::Shadow::default(), snap: false,
                        })
                );
            }
        }
    }

    container(col).width(Length::Fixed(220.0))
        .style(move |_t: &Theme| Style {
            background: Some(Background::Color(tc.colors.bg_elevated)),
            border: Border { color: tc.colors.border, width: 1.0, radius: Radii::MD.into() },
            text_color: None,
            shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.2), offset: iced::Vector::new(0.0, 4.0), blur_radius: 16.0 },
            snap: false,
        }).into()
}

fn ctx_style(theme: &NyxTheme, status: Status) -> ButtonStyle {
    let colors = &theme.colors;
    let (bg, tc) = match status {
        Status::Active | Status::Disabled => (None, colors.text_primary),
        Status::Hovered => (Some(Background::Color(colors.accent_subtle)), colors.text_primary),
        Status::Pressed => (Some(Background::Color(colors.accent)), colors.text_on_accent),
    };
    ButtonStyle {
        background: bg, text_color: tc,
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::SM.into() },
        shadow: iced::Shadow::default(), snap: false,
    }
}
