//! # Dock — macOS-style bottom dock

use iced::{
    widget::{button, container, row, text, Space},
    Alignment, Border, Background, Color, Element, Length, Padding, Theme,
};
use iced::widget::button::{Status, Style as ButtonStyle};
use nyx_widgets::theme::{NyxTheme, Spacing, Radii};

#[derive(Debug, Clone)]
pub struct DockItem { pub id: String, pub name: String, pub icon: String, pub pinned: bool, pub running: bool, pub focused: bool }

pub struct DockState { pub items: Vec<DockItem>, pub hovered_index: Option<usize> }

impl DockState {
    pub fn new() -> Self {
        Self {
            items: vec![
                DockItem { id: "nyx-files".into(), name: "Files".into(), icon: "📁".into(), pinned: true, running: false, focused: false },
                DockItem { id: "nyx-browser".into(), name: "Browser".into(), icon: "🌐".into(), pinned: true, running: true, focused: true },
                DockItem { id: "nyx-terminal".into(), name: "Terminal".into(), icon: "⬛".into(), pinned: true, running: true, focused: false },
                DockItem { id: "nyx-editor".into(), name: "Editor".into(), icon: "📝".into(), pinned: true, running: false, focused: false },
                DockItem { id: "nyx-settings".into(), name: "Settings".into(), icon: "⚙️".into(), pinned: true, running: false, focused: false },
                DockItem { id: "nyx-store".into(), name: "Store".into(), icon: "🛍️".into(), pinned: true, running: false, focused: false },
                DockItem { id: "nyx-music".into(), name: "Music".into(), icon: "🎵".into(), pinned: true, running: false, focused: false },
                DockItem { id: "nyx-mail".into(), name: "Mail".into(), icon: "✉️".into(), pinned: true, running: false, focused: false },
            ],
            hovered_index: None,
        }
    }
    pub fn update(&mut self, msg: DockMessage) {
        match msg {
            DockMessage::ClickItem(id) => {
                tracing::info!("Dock: clicked {}", id);
                if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                    item.running = true; item.focused = true;
                }
            }
            DockMessage::HoverItem(i) => { self.hovered_index = Some(i); }
            DockMessage::UnhoverItem => { self.hovered_index = None; }
            DockMessage::ToggleStartMenu => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum DockMessage { ClickItem(String), HoverItem(usize), UnhoverItem, ToggleStartMenu }

pub fn view<'a>(state: &DockState, theme: &NyxTheme) -> Element<'a, DockMessage> {
    let colors = &theme.colors;
    
    // Start Button (Mac/Windows hybrid style)
    let start_theme = theme.clone();
    let start_btn = button(
        container(text("🌌").size(32.0)).center_x(Length::Fixed(52.0)).center_y(Length::Fixed(52.0))
    )
    .padding(Padding::from([2.0, 2.0]))
    .on_press(DockMessage::ToggleStartMenu)
    .style(move |_t: &Theme, status: Status| {
        let bg = match status {
            Status::Active | Status::Disabled => None,
            Status::Hovered => Some(Background::Color(Color::from_rgba(start_theme.colors.accent.r, start_theme.colors.accent.g, start_theme.colors.accent.b, 0.15))),
            Status::Pressed => Some(Background::Color(Color::from_rgba(start_theme.colors.accent.r, start_theme.colors.accent.g, start_theme.colors.accent.b, 0.3))),
        };
        ButtonStyle {
            background: bg,
            text_color: start_theme.colors.accent,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::LG.into() },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    });

    let start_col = iced::widget::column![start_btn]
        .align_x(Alignment::Center)
        .push(Space::new().width(Length::Fixed(5.0)).height(Length::Fixed(5.0)));

    // Separator line
    let sep_theme = theme.clone();
    let sep = container(Space::new().width(Length::Fixed(1.0)).height(Length::Fixed(36.0)))
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(sep_theme.colors.divider)),
            ..Default::default()
        })
        .align_y(Alignment::Center)
        .padding(Padding::from(Spacing::XS));

    let mut dock_row = row![start_col, sep].spacing(Spacing::XXS).align_y(Alignment::End);

    for (i, item) in state.items.iter().enumerate() {
        let is_hovered = state.hovered_index == Some(i);
        let is_running = item.running;
        let is_focused = item.focused;
        let item_id = item.id.clone();
        let tc = theme.clone();
        let icon_size = if is_hovered { 42.0 } else { 36.0 };

        let icon_btn = button(
            container(text(item.icon.clone()).size(icon_size)).center_x(Length::Fixed(52.0)).center_y(Length::Fixed(52.0))
        )
        .padding(Padding::from([2.0, 2.0]))
        .on_press(DockMessage::ClickItem(item_id))
        .style(move |_t: &Theme, status: Status| {
            let bg = match status {
                Status::Active | Status::Disabled => if is_focused { Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))) } else { None },
                Status::Hovered => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
                Status::Pressed => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.15))),
            };
            ButtonStyle { background: bg, text_color: tc.colors.text_primary, border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::LG.into() }, shadow: iced::Shadow::default(), snap: false }
        });

        let dot_color = if is_focused { colors.accent } else { colors.text_tertiary };
        let mut item_col = iced::widget::column![icon_btn].align_x(Alignment::Center).spacing(2.0);
        if is_running {
            item_col = item_col.push(
                container(Space::new().width(Length::Fixed(5.0)).height(Length::Fixed(5.0)))
                    .style(move |_t: &Theme| iced::widget::container::Style {
                        background: Some(Background::Color(dot_color)),
                        border: Border { radius: Radii::FULL.into(), ..Border::default() },
                        ..Default::default()
                    })
            );
        } else {
            item_col = item_col.push(Space::new().width(Length::Fixed(5.0)).height(Length::Fixed(5.0)));
        }
        dock_row = dock_row.push(item_col);
    }

    let dock_bg = colors.dock_bg;
    let dock_border = colors.border;
    let dock_container = container(
        container(dock_row).padding(Padding::from([Spacing::XS, Spacing::MD]))
            .style(move |_t: &Theme| iced::widget::container::Style {
                background: Some(Background::Color(dock_bg)),
                border: Border { color: dock_border, width: 1.0, radius: Radii::XL.into() },
                text_color: None,
                shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.25), offset: iced::Vector::new(0.0, 4.0), blur_radius: 20.0 },
                snap: false,
            })
    ).width(Length::Fill).center_x(Length::Fill).padding(Padding::from(Spacing::XS));

    dock_container.into()
}
