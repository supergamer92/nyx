//! # Nyx Toast Notifications

use iced::widget::{container, text, Row};
use iced::{Element, Length, Border, Background, Color, Theme, Alignment};
use iced::widget::container::Style;
use crate::theme::{NyxTheme, Spacing, Typography, Radii};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind { Info, Success, Warning, Error }

impl ToastKind {
    pub fn icon(&self) -> &'static str {
        match self { Self::Info => "ℹ️", Self::Success => "✅", Self::Warning => "⚠️", Self::Error => "❌" }
    }
    pub fn accent_color(&self, theme: &NyxTheme) -> Color {
        match self { Self::Info => theme.colors.info, Self::Success => theme.colors.success, Self::Warning => theme.colors.warning, Self::Error => theme.colors.error }
    }
}

#[derive(Debug, Clone)]
pub struct Toast { pub title: String, pub message: Option<String>, pub kind: ToastKind, pub duration_secs: f32 }

pub fn nyx_toast<'a, Message: 'a>(toast: &Toast, theme: &NyxTheme) -> Element<'a, Message> {
    let _accent = toast.kind.accent_color(theme);
    let tc = theme.clone();

    let mut content = Row::new().spacing(Spacing::SM).align_y(Alignment::Center)
        .push(text(toast.kind.icon()).size(Typography::SIZE_BODY_LG));

    let mut text_col = iced::widget::column![];
    text_col = text_col.push(text(toast.title.clone()).size(Typography::SIZE_BODY).color(theme.colors.text_primary));
    if let Some(ref msg) = toast.message {
        text_col = text_col.push(text(msg.clone()).size(Typography::SIZE_BODY_SM).color(theme.colors.text_secondary));
    }
    content = content.push(text_col);

    container(content).width(Length::Fixed(340.0)).padding(Spacing::MD)
        .style(move |_t: &Theme| Style {
            background: Some(Background::Color(tc.colors.bg_elevated)),
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radii::LG.into() },
            text_color: Some(tc.colors.text_primary),
            shadow: iced::Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.2), offset: iced::Vector::new(0.0, 8.0), blur_radius: 24.0 },
            snap: false,
        }).into()
}
