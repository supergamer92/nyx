//! # Nyx Shell — Desktop Shell for Nyx OS

mod top_bar;
mod dock;
mod launcher;
mod notifications;
mod control_center;

use iced::{
    Element, Length, Theme, Subscription, Color,
    widget::{column, container, text, Space},
};
use nyx_widgets::theme::NyxTheme;
use std::time::Duration;

fn main() -> iced::Result {
    tracing_subscriber::fmt().with_env_filter("nyx_shell=info").init();
    tracing::info!("Starting Nyx Shell...");

    iced::application(NyxShell::default, NyxShell::update, NyxShell::view)
        .title("Nyx Shell")
        .subscription(NyxShell::subscription)
        .window_size((1280.0, 800.0))
        .theme(NyxShell::theme)
        .antialiasing(true)
        .run()
}

struct NyxShell {
    theme: NyxTheme,
    clock_text: String,
    top_bar: top_bar::TopBarState,
    dock: dock::DockState,
    launcher: launcher::LauncherState,
    notifications: notifications::NotificationState,
    control_center: control_center::ControlCenterState,
    launcher_visible: bool,
    control_center_visible: bool,
    #[allow(dead_code)]
    notifications_visible: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    TopBar(top_bar::TopBarMessage),
    Dock(dock::DockMessage),
    Launcher(launcher::LauncherMessage),
    #[allow(dead_code)]
    Notification(notifications::NotificationMessage),
    ControlCenter(control_center::ControlCenterMessage),
    ToggleLauncher,
    #[allow(dead_code)]
    ToggleControlCenter,
    #[allow(dead_code)]
    ToggleNotifications,
}

impl Default for NyxShell {
    fn default() -> Self {
        Self {
            theme: NyxTheme::dark(),
            clock_text: chrono::Local::now().format("%H:%M").to_string(),
            top_bar: top_bar::TopBarState::new(),
            dock: dock::DockState::new(),
            launcher: launcher::LauncherState::new(),
            notifications: notifications::NotificationState::new(),
            control_center: control_center::ControlCenterState::new(),
            launcher_visible: false,
            control_center_visible: false,
            notifications_visible: false,
        }
    }
}

impl NyxShell {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => { self.clock_text = chrono::Local::now().format("%H:%M").to_string(); }
            Message::TopBar(msg) => { self.top_bar.update(msg); }
            Message::Dock(msg) => { self.dock.update(msg); }
            Message::Launcher(msg) => {
                let close = matches!(msg, launcher::LauncherMessage::Close);
                self.launcher.update(msg);
                if close { self.launcher_visible = false; }
            }
            Message::Notification(msg) => { self.notifications.update(msg); }
            Message::ControlCenter(msg) => { self.control_center.update(msg); }
            Message::ToggleLauncher => {
                self.launcher_visible = !self.launcher_visible;
                self.control_center_visible = false;
                self.notifications_visible = false;
            }
            Message::ToggleControlCenter => {
                self.control_center_visible = !self.control_center_visible;
                self.launcher_visible = false;
                self.notifications_visible = false;
            }
            Message::ToggleNotifications => {
                self.notifications_visible = !self.notifications_visible;
                self.launcher_visible = false;
                self.control_center_visible = false;
            }
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<Message> {
        let theme = &self.theme;
        let top = top_bar::view(&self.top_bar, &self.clock_text, theme);

        let desktop = container(
            if self.launcher_visible {
                launcher::view(&self.launcher, theme).map(Message::Launcher)
            } else if self.control_center_visible {
                control_center::view(&self.control_center, theme).map(Message::ControlCenter)
            } else {
                container(
                    column![
                        Space::new().height(Length::Fill),
                        container(text("Nyx OS").size(48.0).color(Color::from_rgba(1.0, 1.0, 1.0, 0.15))).center_x(Length::Fill),
                        container(text("v0.1.0-dev").size(16.0).color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))).center_x(Length::Fill),
                        Space::new().height(Length::Fill),
                    ]
                ).width(Length::Fill).height(Length::Fill).into()
            },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(nyx_widgets::theme::hex("#0f0f14"))),
            ..Default::default()
        });

        let dock_bar = dock::view(&self.dock, theme);

        let shell = column![
            top.map(Message::TopBar),
            desktop,
            dock_bar.map(Message::Dock),
        ];

        container(shell).width(Length::Fill).height(Length::Fill).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
