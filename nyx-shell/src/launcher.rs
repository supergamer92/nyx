//! # Hybrid Start Menu / App Launcher
//!
//! A floating Windows 11/macOS hybrid Start Menu. Pops up above the bottom dock
//! with search, pinned apps, recent items, and user profile/power actions.

use iced::{
    widget::{button, column, container, row, text, text_input, scrollable, Space},
    Alignment, Border, Background, Color, Element, Length, Padding, Theme,
};
use iced::widget::button::{Status, Style as ButtonStyle};
use nyx_widgets::theme::{NyxTheme, Spacing, Typography, Radii};

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct RecentFile {
    pub name: String,
    pub path: String,
    pub icon: String,
    pub time: String,
}

pub struct LauncherState {
    pub apps: Vec<AppEntry>,
    pub recents: Vec<RecentFile>,
    pub search_query: String,
    pub user_name: String,
    pub user_avatar: String,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            apps: default_apps(),
            recents: default_recents(),
            search_query: String::new(),
            user_name: "SuperGamer92".into(),
            user_avatar: "👤".into(),
        }
    }

    pub fn update(&mut self, msg: LauncherMessage) {
        match msg {
            LauncherMessage::SearchChanged(q) => {
                self.search_query = q;
            }
            LauncherMessage::LaunchApp(id) => {
                tracing::info!("Launching: {}", id);
            }
            LauncherMessage::OpenFile(path) => {
                tracing::info!("Opening file: {}", path);
            }
            LauncherMessage::PowerAction(action) => {
                tracing::info!("Power action: {}", action);
            }
            LauncherMessage::Close => {}
        }
    }

    pub fn filtered_apps(&self) -> Vec<&AppEntry> {
        self.apps.iter().filter(|a| {
            self.search_query.is_empty()
                || a.name.to_lowercase().contains(&self.search_query.to_lowercase())
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub enum LauncherMessage {
    SearchChanged(String),
    LaunchApp(String),
    OpenFile(String),
    PowerAction(String),
    Close,
}

pub fn view<'a>(state: &LauncherState, theme: &'a NyxTheme) -> Element<'a, LauncherMessage> {
    let colors = &theme.colors;

    // ── 1. Top Search Bar ──
    let search = container(
        text_input("Type to search apps, files, settings...", &state.search_query)
            .on_input(LauncherMessage::SearchChanged)
            .padding(Spacing::SM)
            .size(Typography::SIZE_BODY)
    )
    .width(Length::Fill);

    // ── 2. Pinned Apps Grid (3 columns, compact mac-style grid) ──
    let filtered = state.filtered_apps();
    let mut apps_col = column![].spacing(Spacing::XS);
    let mut current_row = row![].spacing(Spacing::XS);
    let mut count = 0;

    for app in &filtered {
        let app_id = app.id.clone();
        let tc = theme.clone();

        let app_btn = button(
            row![
                container(text(app.icon.clone()).size(24.0)).padding(Spacing::XXS),
                column![
                    text(app.name.clone()).size(Typography::SIZE_BODY_SM).color(colors.text_primary),
                    text(app.category.clone()).size(Typography::SIZE_CAPTION).color(colors.text_tertiary),
                ].spacing(2.0),
            ]
            .spacing(Spacing::SM)
            .align_y(Alignment::Center)
        )
        .width(Length::Fixed(190.0))
        .padding(Spacing::XS)
        .on_press(LauncherMessage::LaunchApp(app_id))
        .style(move |_t: &Theme, status: Status| {
            let bg = match status {
                Status::Active | Status::Disabled => None,
                Status::Hovered => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))),
                Status::Pressed => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
            };
            ButtonStyle {
                background: bg,
                text_color: tc.colors.text_primary,
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
                shadow: iced::Shadow::default(),
                snap: false,
            }
        });

        current_row = current_row.push(app_btn);
        count += 1;

        if count >= 3 {
            apps_col = apps_col.push(current_row);
            current_row = row![].spacing(Spacing::XS);
            count = 0;
        }
    }
    if count > 0 {
        apps_col = apps_col.push(current_row);
    }

    let apps_section = column![
        text("Pinned Apps")
            .size(Typography::SIZE_BODY_SM)
            .color(colors.text_secondary),
        Space::new().height(Length::Fixed(Spacing::XS)),
        container(scrollable(apps_col)).height(Length::Fixed(200.0)),
    ];

    // ── 3. Recent Items Section (Windows-style) ──
    let mut recents_col = column![].spacing(Spacing::XXS);
    for item in &state.recents {
        let path = item.path.clone();
        let tc = theme.clone();

        let item_btn = button(
            row![
                text(item.icon.clone()).size(20.0),
                column![
                    text(item.name.clone()).size(Typography::SIZE_BODY_SM).color(colors.text_primary),
                    text(item.path.clone()).size(Typography::SIZE_CAPTION).color(colors.text_tertiary),
                ].spacing(2.0),
                Space::new().width(Length::Fill),
                text(item.time.clone()).size(Typography::SIZE_CAPTION).color(colors.text_tertiary),
            ]
            .spacing(Spacing::SM)
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(Spacing::XS)
        .on_press(LauncherMessage::OpenFile(path))
        .style(move |_t: &Theme, status: Status| {
            let bg = match status {
                Status::Active | Status::Disabled => None,
                Status::Hovered => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))),
                Status::Pressed => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
            };
            ButtonStyle {
                background: bg,
                text_color: tc.colors.text_primary,
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::MD.into() },
                shadow: iced::Shadow::default(),
                snap: false,
            }
        });

        recents_col = recents_col.push(item_btn);
    }

    let recents_section = column![
        text("Recommended Files")
            .size(Typography::SIZE_BODY_SM)
            .color(colors.text_secondary),
        Space::new().height(Length::Fixed(Spacing::XS)),
        recents_col,
    ];

    // ── 4. Bottom Footer (User profile + Power Menu) ──
    let footer_bg = colors.bg_surface;
    let divider = colors.divider;

    let footer = container(
        row![
            // User Avatar & Name
            row![
                text(state.user_avatar.clone()).size(24.0),
                text(state.user_name.clone()).size(Typography::SIZE_BODY_SM).color(colors.text_primary),
            ]
            .spacing(Spacing::SM)
            .align_y(Alignment::Center),
            
            Space::new().width(Length::Fill),
            
            // Power Button
            button(text("⚙️").size(18.0))
                .padding(Spacing::XS)
                .on_press(LauncherMessage::PowerAction("settings".into()))
                .style(move |_t, _s| ghost_btn(colors.text_secondary)),
            button(text("🔒").size(18.0))
                .padding(Spacing::XS)
                .on_press(LauncherMessage::PowerAction("lock".into()))
                .style(move |_t, _s| ghost_btn(colors.text_secondary)),
            button(text("⏻").size(18.0))
                .padding(Spacing::XS)
                .on_press(LauncherMessage::PowerAction("shutdown".into()))
                .style(move |_t, _s| ghost_btn(colors.error)),
        ]
        .align_y(Alignment::Center)
    )
    .padding(Spacing::SM)
    .width(Length::Fill)
    .style(move |_t: &Theme| iced::widget::container::Style {
        background: Some(Background::Color(footer_bg)),
        border: Border { color: divider, width: 0.0, radius: Radii::LG.into() },
        ..Default::default()
    });

    // ── Assemble Start Menu Layout ──
    let content = column![
        search,
        Space::new().height(Length::Fixed(Spacing::MD)),
        apps_section,
        Space::new().height(Length::Fixed(Spacing::MD)),
        recents_section,
        Space::new().height(Length::Fill),
        footer,
    ]
    .spacing(Spacing::XS);

    // Floating card container (Windows 11 style popup layout)
    let card_bg = colors.bg_elevated;
    let card_border = colors.border;

    container(
        container(content)
            .width(Length::Fixed(600.0))
            .height(Length::Fixed(550.0))
            .padding(Spacing::MD)
            .style(move |_t: &Theme| iced::widget::container::Style {
                background: Some(Background::Color(card_bg)),
                border: Border { color: card_border, width: 1.0, radius: Radii::XL.into() },
                text_color: None,
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.45),
                    offset: iced::Vector::new(0.0, 16.0),
                    blur_radius: 48.0,
                },
                snap: false,
            })
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .align_bottom(Length::Fill)
    .padding(Padding::from(Spacing::LG))
    .into()
}

fn ghost_btn(color: Color) -> ButtonStyle {
    ButtonStyle {
        background: None,
        text_color: color,
        border: Border::default(),
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

fn default_apps() -> Vec<AppEntry> {
    vec![
        AppEntry { id: "files".into(), name: "Files".into(), icon: "📁".into(), category: "System Tool".into() },
        AppEntry { id: "browser".into(), name: "Web Browser".into(), icon: "🌐".into(), category: "Productivity".into() },
        AppEntry { id: "terminal".into(), name: "Terminal".into(), icon: "⬛".into(), category: "System Tool".into() },
        AppEntry { id: "editor".into(), name: "Text Editor".into(), icon: "📝".into(), category: "Productivity".into() },
        AppEntry { id: "settings".into(), name: "Settings".into(), icon: "⚙️".into(), category: "Configuration".into() },
        AppEntry { id: "music".into(), name: "Music".into(), icon: "🎵".into(), category: "Media Player".into() },
    ]
}

fn default_recents() -> Vec<RecentFile> {
    vec![
        RecentFile { name: "Project Draft.md".into(), path: "~/Documents/".into(), icon: "📄".into(), time: "5m ago".into() },
        RecentFile { name: "wallpaper.png".into(), path: "~/Pictures/".into(), icon: "🖼️".into(), time: "1h ago".into() },
        RecentFile { name: "build-iso.yml".into(), path: "~/.github/workflows/".into(), icon: "⚙️".into(), time: "Yesterday".into() },
    ]
}
