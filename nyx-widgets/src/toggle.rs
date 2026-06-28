//! # Nyx Toggle Switch
//!
//! An iOS/macOS-style toggle switch with smooth spring animation.

use iced::widget::canvas::{self, Canvas, Frame, Path};
use iced::{Element, Length, Size, Rectangle, Point, Color, mouse, Theme, Renderer};
use crate::theme::{NyxTheme, Radii};

/// Toggle state
#[derive(Debug, Clone, Copy)]
pub struct ToggleState {
    pub is_on: bool,
    /// Animation progress 0.0 (off) to 1.0 (on)
    pub animation_t: f32,
}

impl ToggleState {
    pub fn new(is_on: bool) -> Self {
        Self {
            is_on,
            animation_t: if is_on { 1.0 } else { 0.0 },
        }
    }

    /// Step the spring animation toward the target
    pub fn tick(&mut self, dt: f32) {
        let target = if self.is_on { 1.0 } else { 0.0 };
        // Simple ease — in production, use real spring physics
        let speed = 8.0;
        self.animation_t += (target - self.animation_t) * speed * dt;
        // Snap when close
        if (self.animation_t - target).abs() < 0.01 {
            self.animation_t = target;
        }
    }
}

/// Toggle switch dimensions
const TOGGLE_WIDTH: f32 = 48.0;
const TOGGLE_HEIGHT: f32 = 28.0;
const KNOB_PADDING: f32 = 3.0;
const KNOB_SIZE: f32 = TOGGLE_HEIGHT - KNOB_PADDING * 2.0;

/// Draw a toggle switch as a canvas element
pub fn nyx_toggle<'a, Message: 'a>(
    state: &ToggleState,
    theme: &NyxTheme,
) -> Element<'a, Message> {
    let t = state.animation_t;
    let colors = theme.colors.clone();

    Canvas::new(ToggleRenderer { t, colors })
        .width(Length::Fixed(TOGGLE_WIDTH))
        .height(Length::Fixed(TOGGLE_HEIGHT))
        .into()
}

struct ToggleRenderer {
    t: f32,
    colors: crate::theme::ColorPalette,
}

impl<Message> canvas::Program<Message, Theme, Renderer> for ToggleRenderer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        let radius = TOGGLE_HEIGHT / 2.0;

        // Track color — interpolate between off (grey) and on (accent)
        let track_color = crate::theme::mix(
            self.colors.border,
            self.colors.accent,
            self.t,
        );

        // Draw track (rounded rectangle via two circles + rect)
        let track = Path::rectangle(
            Point::new(0.0, 0.0),
            Size::new(TOGGLE_WIDTH, TOGGLE_HEIGHT),
        );
        frame.fill(&track, track_color);

        // Draw knob
        let knob_x = KNOB_PADDING + self.t * (TOGGLE_WIDTH - KNOB_SIZE - KNOB_PADDING * 2.0);
        let knob_center = Point::new(
            knob_x + KNOB_SIZE / 2.0,
            TOGGLE_HEIGHT / 2.0,
        );
        let knob = Path::circle(knob_center, KNOB_SIZE / 2.0);
        frame.fill(&knob, Color::WHITE);

        vec![frame.into_geometry()]
    }
}
