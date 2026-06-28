//! # App Launcher — full-screen overlay with grid and search

use iced::{
    widget::{button, column, container, row, text, text_input, scrollable, Space},
    Alignment, Border, Background, Color, Element, Length, Padding, Theme,
};
use iced::widget::button::{Status, Style as ButtonStyle};
use nyx_widgets::theme::{NyxTheme, Spacing, Typography, Radii};

#[derive(Debug, Clone)]
pub struct AppEntry { pub id: String, pub name: String, pub icon: String, pub category: String, pub description: String }

pub struct LauncherState { pub apps: Vec<AppEntry>, pub search_query: String, pub selected_category: String }

impl LauncherState {
    pub fn new() -> Self { Self { apps: default_apps(), search_query: String::new(), selected_category: "All".into() } }
    pub fn update(&mut self, msg: LauncherMessage) {
        match msg {
            LauncherMessage::SearchChanged(q) => { self.search_query = q; }
            LauncherMessage::LaunchApp(id) => { tracing::info!("Launching: {}", id); }
            LauncherMessage::SelectCategory(c) => { self.selected_category = c; }
            LauncherMessage::Close => {}
        }
    }
    pub fn filtered_apps(&self) -> Vec<&AppEntry> {
        self.apps.iter().filter(|a| {
            let ms = self.search_query.is_empty() || a.name.to_lowercase().contains(&self.search_query.to_lowercase());
            let mc = self.selected_category == "All" || a.category == self.selected_category;
            ms && mc
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub enum LauncherMessage { SearchChanged(String), LaunchApp(String), SelectCategory(String), Close }

pub fn view<'a>(state: &LauncherState, theme: &NyxTheme) -> Element<'a, LauncherMessage> {
    let colors = &theme.colors;

    let search = container(
        text_input("Search apps...", &state.search_query).on_input(LauncherMessage::SearchChanged).padding(Spacing::SM).size(Typography::SIZE_BODY_LG)
    ).width(Length::Fixed(400.0)).center_x(Length::Fill);

    let categories = ["All", "System", "Productivity", "Media", "Utilities"];
    let mut cat_row = row![].spacing(Spacing::XS);
    for cat in &categories {
        let is_active = state.selected_category == *cat;
        let tc = theme.clone();
        let cs = cat.to_string();
        let btn = button(text(cat.to_string()).size(Typography::SIZE_BODY_SM))
            .padding(Padding::from([Spacing::XXS, Spacing::SM]))
            .on_press(LauncherMessage::SelectCategory(cs))
            .style(move |_t: &Theme, status: Status| {
                let c = &tc.colors;
                if is_active {
                    ButtonStyle { background: Some(Background::Color(c.accent)), text_color: c.text_on_accent, border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::FULL.into() }, shadow: iced::Shadow::default(), snap: false }
                } else {
                    let (bg, t) = match status {
                        Status::Active | Status::Disabled => (Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))), c.text_secondary),
                        Status::Hovered => (Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))), c.text_primary),
                        Status::Pressed => (Some(Background::Color(c.accent_subtle)), c.accent),
                    };
                    ButtonStyle { background: bg, text_color: t, border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::FULL.into() }, shadow: iced::Shadow::default(), snap: false }
                }
            });
        cat_row = cat_row.push(btn);
    }
    let cat_container = container(cat_row).center_x(Length::Fill);

    let filtered = state.filtered_apps();
    let cols = 6;
    let mut grid = column![].spacing(Spacing::SM);
    let mut current_row = row![].spacing(Spacing::SM);
    let mut col_count = 0;

    for app in &filtered {
        let app_id = app.id.clone();
        let tc = theme.clone();
        let text_color = colors.text_primary;
        let app_btn = button(
            column![
                container(text(app.icon.clone()).size(36.0)).center_x(Length::Fill).center_y(Length::Fixed(48.0)),
                container(text(app.name.clone()).size(Typography::SIZE_BODY_SM).color(text_color)).center_x(Length::Fill),
            ].spacing(Spacing::XXS).align_x(Alignment::Center)
        )
        .width(Length::Fixed(100.0)).height(Length::Fixed(100.0)).padding(Spacing::XS)
        .on_press(LauncherMessage::LaunchApp(app_id))
        .style(move |_t: &Theme, status: Status| {
            let (bg, t) = match status {
                Status::Active | Status::Disabled => (None, tc.colors.text_primary),
                Status::Hovered => (Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))), tc.colors.text_primary),
                Status::Pressed => (Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.14))), tc.colors.text_primary),
            };
            ButtonStyle { background: bg, text_color: t, border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::LG.into() }, shadow: iced::Shadow::default(), snap: false }
        });
        current_row = current_row.push(app_btn);
        col_count += 1;
        if col_count >= cols { grid = grid.push(current_row); current_row = row![].spacing(Spacing::SM); col_count = 0; }
    }
    if col_count > 0 { grid = grid.push(current_row); }

    let grid_container = container(scrollable(container(grid).center_x(Length::Fill))).center_x(Length::Fill).height(Length::Fill);

    let layout = column![
        Space::new().height(Length::Fixed(60.0)),
        search,
        Space::new().height(Length::Fixed(20.0)),
        cat_container,
        Space::new().height(Length::Fixed(24.0)),
        grid_container,
    ].align_x(Alignment::Center);

    container(layout).width(Length::Fill).height(Length::Fill).padding(Spacing::LG)
        .style(|_t: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(0.02, 0.02, 0.04, 0.92))),
            ..Default::default()
        }).into()
}

fn default_apps() -> Vec<AppEntry> {
    vec![
        AppEntry { id: "nyx-files".into(), name: "Files".into(), icon: "📁".into(), category: "System".into(), description: "File manager".into() },
        AppEntry { id: "nyx-browser".into(), name: "Browser".into(), icon: "🌐".into(), category: "Productivity".into(), description: "Web browser".into() },
        AppEntry { id: "nyx-terminal".into(), name: "Terminal".into(), icon: "⬛".into(), category: "System".into(), description: "Terminal emulator".into() },
        AppEntry { id: "nyx-editor".into(), name: "Editor".into(), icon: "📝".into(), category: "Productivity".into(), description: "Text editor".into() },
        AppEntry { id: "nyx-settings".into(), name: "Settings".into(), icon: "⚙️".into(), category: "System".into(), description: "System settings".into() },
        AppEntry { id: "nyx-store".into(), name: "Store".into(), icon: "🛍️".into(), category: "System".into(), description: "App store".into() },
        AppEntry { id: "nyx-music".into(), name: "Music".into(), icon: "🎵".into(), category: "Media".into(), description: "Music player".into() },
        AppEntry { id: "nyx-video".into(), name: "Videos".into(), icon: "🎬".into(), category: "Media".into(), description: "Video player".into() },
        AppEntry { id: "nyx-photos".into(), name: "Photos".into(), icon: "📷".into(), category: "Media".into(), description: "Photo viewer".into() },
        AppEntry { id: "nyx-mail".into(), name: "Mail".into(), icon: "✉️".into(), category: "Productivity".into(), description: "Email client".into() },
        AppEntry { id: "nyx-calendar".into(), name: "Calendar".into(), icon: "📅".into(), category: "Productivity".into(), description: "Calendar".into() },
        AppEntry { id: "nyx-monitor".into(), name: "Monitor".into(), icon: "📊".into(), category: "Utilities".into(), description: "System monitor".into() },
    ]
}
