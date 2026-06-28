//! # Apple-Style Vector Icon Engine
//!
//! Renders premium vector SVG icons embedded into the binary at compile time.

use iced::{Element, Length, Border, Background, Color, Theme};
use iced::widget::{container, svg};
use nyx_widgets::theme::{Radii, hex};

pub fn render_app_icon<'a, Message: 'a>(app_id: &str, size: f32) -> Element<'a, Message> {
    let bg_color = match app_id {
        "nyx-files" | "files" => hex("#1d4ed8"), // Apple blue
        "nyx-browser" | "browser" => hex("#0284c7"), // Apple Safari/sky blue
        "nyx-terminal" | "terminal" => hex("#1e293b"), // Carbon/slate metal
        "nyx-editor" | "editor" => hex("#d97706"), // Warm Amber
        "nyx-settings" | "settings" => hex("#475569"), // Precision steel
        "nyx-store" | "store" => hex("#7c3aed"), // iOS violet
        "nyx-music" | "music" => hex("#db2777"), // Music rose pink
        "nyx-mail" | "mail" => hex("#059669"), // Forest green
        _ => hex("#4f46e5"), // Indigo fallback
    };

    let svg_handle = match app_id {
        "nyx-files" | "files" => svg::Handle::from_memory(include_bytes!("../assets/files.svg").as_ref()),
        "nyx-browser" | "browser" => svg::Handle::from_memory(include_bytes!("../assets/browser.svg").as_ref()),
        "nyx-terminal" | "terminal" => svg::Handle::from_memory(include_bytes!("../assets/terminal.svg").as_ref()),
        "nyx-editor" | "editor" => svg::Handle::from_memory(include_bytes!("../assets/editor.svg").as_ref()),
        "nyx-settings" | "settings" => svg::Handle::from_memory(include_bytes!("../assets/settings.svg").as_ref()),
        "nyx-store" | "store" => svg::Handle::from_memory(include_bytes!("../assets/store.svg").as_ref()),
        "nyx-music" | "music" => svg::Handle::from_memory(include_bytes!("../assets/music.svg").as_ref()),
        "nyx-mail" | "mail" => svg::Handle::from_memory(include_bytes!("../assets/mail.svg").as_ref()),
        _ => svg::Handle::from_memory(include_bytes!("../assets/settings.svg").as_ref()),
    };

    let padding_val = size * 0.25;
    let svg_inner = svg(svg_handle)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_t, _s| iced::widget::svg::Style {
            color: Some(Color::WHITE),
        });

    container(svg_inner)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .padding(padding_val)
        .style(move |_t: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.08), // Soft hairline edge highlight
                width: 1.0,
                radius: Radii::LG.into(), // squircle corner radius
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            text_color: None,
            snap: false,
        })
        .into()
}
