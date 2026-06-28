//! # Nyx Progress Indicators

use iced::widget::progress_bar;
use iced::{Element, Border, Background, Theme};
use iced::widget::progress_bar::Style;
use crate::theme::{NyxTheme, Radii};

/// Create a styled progress bar
pub fn nyx_progress_bar<'a, Message: 'a>(value: f32, theme: &NyxTheme) -> Element<'a, Message> {
    let tc = theme.clone();
    progress_bar(0.0..=100.0, value)
        .style(move |_theme: &Theme| Style {
            background: Background::Color(tc.colors.border),
            bar: Background::Color(tc.colors.accent),
            border: Border { radius: Radii::FULL.into(), ..Border::default() },
        })
        .into()
}
