//! # Notification Center

use iced::{
    widget::{button, column, container, row, text, scrollable, Space},
    Alignment, Border, Background, Color, Element, Length, Padding, Theme,
};
use nyx_widgets::theme::{NyxTheme, Spacing, Typography, Radii};

#[derive(Debug, Clone)]
pub struct Notification { pub id: u64, pub app_name: String, pub app_icon: String, pub title: String, pub body: String, pub time: String, pub read: bool }

pub struct NotificationState { pub notifications: Vec<Notification>, pub do_not_disturb: bool }

impl NotificationState {
    pub fn new() -> Self {
        Self {
            notifications: vec![
                Notification { id: 1, app_name: "Mail".into(), app_icon: "✉️".into(), title: "New message".into(), body: "You have 3 unread messages".into(), time: "2m ago".into(), read: false },
                Notification { id: 2, app_name: "Store".into(), app_icon: "🛍️".into(), title: "Updates available".into(), body: "5 apps can be updated".into(), time: "15m ago".into(), read: false },
                Notification { id: 3, app_name: "System".into(), app_icon: "⚙️".into(), title: "System update ready".into(), body: "Nyx OS 0.1.1 is available".into(), time: "1h ago".into(), read: true },
            ],
            do_not_disturb: false,
        }
    }
    pub fn update(&mut self, msg: NotificationMessage) {
        match msg {
            NotificationMessage::Dismiss(id) => { self.notifications.retain(|n| n.id != id); }
            NotificationMessage::DismissAll => { self.notifications.clear(); }
            NotificationMessage::ToggleDnd => { self.do_not_disturb = !self.do_not_disturb; }
        }
    }
}

#[derive(Debug, Clone)]
pub enum NotificationMessage { Dismiss(u64), DismissAll, ToggleDnd }

pub fn view<'a>(state: &NotificationState, theme: &NyxTheme) -> Element<'a, NotificationMessage> {
    let colors = &theme.colors;
    let header = row![
        text("Notifications").size(Typography::SIZE_HEADING_SM).color(colors.text_primary),
        Space::new().width(Length::Fill),
        button(text("Clear All").size(Typography::SIZE_BODY_SM)).on_press(NotificationMessage::DismissAll).padding(Padding::from([4.0, 8.0]))
            .style(|_t: &Theme, _s| iced::widget::button::Style { background: None, text_color: Color::from_rgb(0.486, 0.361, 0.988), border: Border::default(), shadow: iced::Shadow::default(), snap: false }),
    ].align_y(Alignment::Center);

    let mut list = column![].spacing(Spacing::XS);
    if state.notifications.is_empty() {
        list = list.push(container(column![text("🔔").size(36.0), text("No notifications").size(Typography::SIZE_BODY).color(colors.text_tertiary)].spacing(Spacing::SM).align_x(Alignment::Center)).center_x(Length::Fill).padding(Spacing::XXL));
    } else {
        for notif in &state.notifications {
            let bg_color = if notif.read { colors.bg_surface } else { colors.bg_elevated };
            let card = container(column![
                row![text(notif.app_icon.clone()).size(Typography::SIZE_BODY), text(notif.app_name.clone()).size(Typography::SIZE_CAPTION).color(colors.text_tertiary), Space::new().width(Length::Fill), text(notif.time.clone()).size(Typography::SIZE_CAPTION).color(colors.text_tertiary)].spacing(Spacing::XS).align_y(Alignment::Center),
                text(notif.title.clone()).size(Typography::SIZE_BODY).color(colors.text_primary),
                text(notif.body.clone()).size(Typography::SIZE_BODY_SM).color(colors.text_secondary),
            ].spacing(Spacing::XXS)).padding(Spacing::SM).width(Length::Fill)
            .style(move |_t: &Theme| iced::widget::container::Style { background: Some(Background::Color(bg_color)), border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::LG.into() }, ..Default::default() });
            list = list.push(card);
        }
    }

    let tc = theme.colors.clone();
    container(column![header, Space::new().height(Length::Fixed(Spacing::MD)), scrollable(list).height(Length::Fill)].spacing(Spacing::XS))
        .width(Length::Fixed(360.0)).height(Length::Fill).padding(Spacing::MD)
        .style(move |_t: &Theme| iced::widget::container::Style { background: Some(Background::Color(tc.bg_surface)), border: Border { color: tc.divider, width: 1.0, radius: 0.0.into() }, text_color: None, shadow: iced::Shadow::default(), snap: false }).into()
}
