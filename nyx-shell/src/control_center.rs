//! # Control Center — quick settings panel

use iced::{
    widget::{button, column, container, row, text, slider, Space},
    Alignment, Border, Background, Color, Element, Length, Padding, Theme,
};
use iced::widget::button::{Status, Style as ButtonStyle};
use nyx_widgets::theme::{NyxTheme, Spacing, Typography, Radii};

pub struct ControlCenterState {
    pub wifi_enabled: bool, pub bluetooth_enabled: bool, pub airplane_mode: bool,
    pub do_not_disturb: bool, pub dark_mode: bool,
    pub volume: f32, pub brightness: f32, pub wifi_network: String,
}

impl ControlCenterState {
    pub fn new() -> Self {
        Self { wifi_enabled: true, bluetooth_enabled: true, airplane_mode: false, do_not_disturb: false, dark_mode: true, volume: 75.0, brightness: 80.0, wifi_network: "Home Network".into() }
    }
    pub fn update(&mut self, msg: ControlCenterMessage) {
        match msg {
            ControlCenterMessage::ToggleWifi => self.wifi_enabled = !self.wifi_enabled,
            ControlCenterMessage::ToggleBluetooth => self.bluetooth_enabled = !self.bluetooth_enabled,
            ControlCenterMessage::ToggleAirplane => { self.airplane_mode = !self.airplane_mode; if self.airplane_mode { self.wifi_enabled = false; self.bluetooth_enabled = false; } }
            ControlCenterMessage::ToggleDnd => self.do_not_disturb = !self.do_not_disturb,
            ControlCenterMessage::ToggleDarkMode => self.dark_mode = !self.dark_mode,
            ControlCenterMessage::SetVolume(v) => self.volume = v,
            ControlCenterMessage::SetBrightness(b) => self.brightness = b,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ControlCenterMessage { ToggleWifi, ToggleBluetooth, ToggleAirplane, ToggleDnd, ToggleDarkMode, SetVolume(f32), SetBrightness(f32) }

fn quick_toggle<'a>(icon: &str, label: &str, active: bool, msg: ControlCenterMessage, theme: &NyxTheme) -> Element<'a, ControlCenterMessage> {
    let accent = theme.colors.accent;
    let bg_el = theme.colors.bg_elevated;
    let text_sec = theme.colors.text_secondary;
    let text_pri = theme.colors.text_primary;
    let border_c = theme.colors.border;

    let content = column![text(icon.to_string()).size(20.0), text(label.to_string()).size(Typography::SIZE_CAPTION)].spacing(Spacing::XXS).align_x(Alignment::Center);
    button(container(content).center_x(Length::Fill).center_y(Length::Fill))
        .width(Length::Fill).height(Length::Fixed(72.0)).on_press(msg)
        .style(move |_t: &Theme, status: Status| {
            if active {
                ButtonStyle { background: Some(Background::Color(accent)), text_color: Color::WHITE, border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::LG.into() }, shadow: iced::Shadow::default(), snap: false }
            } else {
                let (bg, tc, bc) = match status {
                    Status::Active | Status::Disabled => (Some(Background::Color(bg_el)), text_sec, border_c),
                    Status::Hovered | Status::Pressed => (Some(Background::Color(Color::from_rgba(accent.r, accent.g, accent.b, 0.15))), text_pri, accent),
                };
                ButtonStyle { background: bg, text_color: tc, border: Border { color: bc, width: 1.0, radius: Radii::LG.into() }, shadow: iced::Shadow::default(), snap: false }
            }
        }).into()
}

pub fn view<'a>(state: &ControlCenterState, theme: &NyxTheme) -> Element<'a, ControlCenterMessage> {
    let colors = &theme.colors;
    let toggle_grid = column![
        row![
            quick_toggle("📶", "Wi-Fi", state.wifi_enabled, ControlCenterMessage::ToggleWifi, theme),
            quick_toggle("📡", "Bluetooth", state.bluetooth_enabled, ControlCenterMessage::ToggleBluetooth, theme),
            quick_toggle("✈️", "Airplane", state.airplane_mode, ControlCenterMessage::ToggleAirplane, theme),
        ].spacing(Spacing::XS),
        row![
            quick_toggle("🔕", "DND", state.do_not_disturb, ControlCenterMessage::ToggleDnd, theme),
            quick_toggle("🌙", "Dark", state.dark_mode, ControlCenterMessage::ToggleDarkMode, theme),
            Space::new().width(Length::Fill),
        ].spacing(Spacing::XS),
    ].spacing(Spacing::XS);

    let wifi_info = if state.wifi_enabled { text(format!("Connected to {}", state.wifi_network)).size(Typography::SIZE_CAPTION).color(colors.text_secondary) }
        else { text("Wi-Fi Off").size(Typography::SIZE_CAPTION).color(colors.text_tertiary) };

    let volume_row = row![text("🔊").size(Typography::SIZE_BODY), slider(0.0..=100.0, state.volume, ControlCenterMessage::SetVolume).step(1.0).width(Length::Fill), text(format!("{}%", state.volume as u32)).size(Typography::SIZE_CAPTION).color(colors.text_secondary).width(Length::Fixed(36.0))].spacing(Spacing::SM).align_y(Alignment::Center);
    let bright_row = row![text("☀️").size(Typography::SIZE_BODY), slider(0.0..=100.0, state.brightness, ControlCenterMessage::SetBrightness).step(1.0).width(Length::Fill), text(format!("{}%", state.brightness as u32)).size(Typography::SIZE_CAPTION).color(colors.text_secondary).width(Length::Fixed(36.0))].spacing(Spacing::SM).align_y(Alignment::Center);

    let content = column![toggle_grid, wifi_info, Space::new().height(Length::Fixed(Spacing::SM)), volume_row, bright_row].spacing(Spacing::SM);

    let panel_bg = colors.bg_elevated;
    let panel_border = colors.border;
    container(
        container(content).width(Length::Fixed(340.0)).padding(Spacing::MD)
            .style(move |_t: &Theme| iced::widget::container::Style {
                background: Some(Background::Color(panel_bg)), border: Border { color: panel_border, width: 1.0, radius: Radii::XL.into() }, text_color: None,
                shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.3), offset: iced::Vector::new(0.0, 8.0), blur_radius: 32.0 }, snap: false,
            })
    ).width(Length::Fill).height(Length::Fill).padding(Padding::from([Spacing::XL, Spacing::MD])).align_right(Length::Fill).into()
}
