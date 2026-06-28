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

pub fn render_system_icon<'a, Message: 'a>(icon_id: &str, size: f32, color: Color) -> Element<'a, Message> {
    let svg_str = match icon_id {
        "nyx-logo" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a9 9 0 1 0 9 9 9.75 9.75 0 0 0-.67-3.47 6.75 6.75 0 0 1-11.85-6.86A9.78 9.78 0 0 0 12 3Z"/></svg>"#,
        "wifi" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h.01M8.5 16.5a5 5 0 0 1 7 0M5 13a10 10 0 0 1 14 0M1.5 9.5a15 15 0 0 1 21 0"/></svg>"#,
        "volume" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>"#,
        "battery" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="1" y="6" width="18" height="12" rx="2" ry="2"/><line x1="23" y1="11" x2="23" y2="13"/></svg>"#,
        "dnd" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13.73 21a2 2 0 0 1-3.46 0M18.63 13A17.89 17.89 0 0 1 18 8M6.26 6.26A5.86 5.86 0 0 0 6 8v7a2 2 0 0 0-2 2h14a2 2 0 0 0-.18-1M2 2l20 20"/></svg>"#,
        "settings" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#,
        "lock" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"#,
        "power" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18.36 6.64a9 9 0 1 1-12.73 0M12 2v10"/></svg>"#,
        "cpu" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 9h6v6H9zM9 1v3M15 1v3M9 20v3M15 20v3M20 9h3M20 15h3M1 9h3M1 15h3"/></svg>"#,
        "ram" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18M18.7 8l-5.1 5.2-2.8-2.7L7 14.3"/></svg>"#,
        "weather-sun" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/></svg>"#,
        "doc" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4M10 9h8M10 13h8M10 17h8"/></svg>"#,
        "img" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>"#,
        "config" => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>"#,
        _ => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/></svg>"#,
    };

    let svg_handle = svg::Handle::from_memory(svg_str.as_bytes());
    svg(svg_handle)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_t, _s| iced::widget::svg::Style {
            color: Some(color),
        })
        .into()
}
