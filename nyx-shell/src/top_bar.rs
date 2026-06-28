//! # Top Bar — app menu (left), clock (center), system tray (right)

use iced::{
    widget::{button, container, row, text, Space},
    Alignment, Border, Background, Color, Element, Length, Padding, Theme,
};
use iced::widget::button::{Status, Style as ButtonStyle};
use nyx_widgets::theme::{NyxTheme, Spacing, Typography, Radii};

pub struct TopBarState {
    pub battery_percent: u8,
    pub wifi_connected: bool,
    pub volume: f32,
    pub do_not_disturb: bool,
}

impl TopBarState {
    pub fn new() -> Self {
        Self { battery_percent: 85, wifi_connected: true, volume: 0.75, do_not_disturb: false }
    }
    pub fn update(&mut self, msg: TopBarMessage) {
        match msg {
            TopBarMessage::ToggleDnd => { self.do_not_disturb = !self.do_not_disturb; }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum TopBarMessage { ToggleDnd, OpenActivities, OpenControlCenter, OpenNotifications }

fn btn_style(theme: &NyxTheme, status: Status) -> ButtonStyle {
    let colors = &theme.colors;
    let (bg, tc) = match status {
        Status::Active | Status::Disabled => (None, colors.text_primary),
        Status::Hovered => (Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))), colors.text_primary),
        Status::Pressed => (Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12))), colors.text_primary),
    };
    ButtonStyle {
        background: bg, text_color: tc,
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::SM.into() },
        shadow: iced::Shadow::default(), snap: false,
    }
}

pub fn view<'a>(state: &TopBarState, clock: &str, theme: &NyxTheme) -> Element<'a, TopBarMessage> {
    let colors = &theme.colors;
    let t1 = theme.clone();
    let left = button(text("Activities").size(Typography::SIZE_BODY_SM))
        .padding(Padding::from([4.0, 12.0]))
        .on_press(TopBarMessage::OpenActivities)
        .style(move |_t: &Theme, s: Status| btn_style(&t1, s));

    let t2 = theme.clone();
    let center = button(text(clock.to_string()).size(Typography::SIZE_BODY).color(colors.text_primary))
        .padding(Padding::from([4.0, 12.0]))
        .on_press(TopBarMessage::OpenNotifications)
        .style(move |_t: &Theme, s: Status| btn_style(&t2, s));

    let wifi = if state.wifi_connected { "📶" } else { "📵" };
    let vol = if state.volume > 0.5 { "🔊" } else if state.volume > 0.0 { "🔉" } else { "🔇" };
    let bat = if state.battery_percent > 50 { "🔋" } else { "🪫" };
    let dnd = if state.do_not_disturb { "🔕" } else { "" };
    let tray = format!("{}{} {} {}%", dnd, wifi, vol, state.battery_percent);

    let t3 = theme.clone();
    let right = button(text(tray).size(Typography::SIZE_BODY_SM))
        .padding(Padding::from([4.0, 12.0]))
        .on_press(TopBarMessage::OpenControlCenter)
        .style(move |_t: &Theme, s: Status| btn_style(&t3, s));

    let bar = row![left, Space::new().width(Length::Fill), center, Space::new().width(Length::Fill), right]
        .align_y(Alignment::Center).padding(Padding::from([0.0, Spacing::SM]));

    let panel_bg = colors.panel_bg;
    let border_color = colors.divider;
    container(bar).width(Length::Fill).height(Length::Fixed(32.0))
        .style(move |_t: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(panel_bg)),
            border: Border { color: border_color, width: 0.0, radius: 0.0.into() },
            text_color: None, shadow: iced::Shadow::default(), snap: false,
        }).into()
}
