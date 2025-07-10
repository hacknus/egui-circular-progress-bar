use egui::{
    pos2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2,
    Widget, WidgetInfo, WidgetType,
};
use std::f32::consts::{FRAC_PI_2, TAU};

/// A circular progress bar widget for egui
pub struct CircularProgressBar {
    progress: f32,
    size: Option<f32>,
    text: Option<String>,
    animate: bool,
}

impl CircularProgressBar {
    /// Create a new circular progress bar with the given progress (0.0 to 1.0)
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            size: None,
            text: None,
            animate: false,
        }
    }

    /// Set the size (diameter) of the circular progress bar
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Add text to display in the center of the progress bar
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Enable animation for indeterminate progress
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Create an indeterminate progress bar (animated)
    pub fn indeterminate() -> Self {
        Self {
            progress: 0.0,
            size: None,
            text: None,
            animate: true,
        }
    }
}

impl Widget for CircularProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = self.size.unwrap_or(ui.spacing().interact_size.y);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());

        if ui.is_rect_visible(rect) {
            self.paint_at(ui, rect);
        }

        if let Some(text) = &self.text {
            response.widget_info(|| WidgetInfo::labeled(WidgetType::ProgressIndicator, true, text));
        } else {
            response.widget_info(|| {
                WidgetInfo::labeled(
                    WidgetType::ProgressIndicator,
                    true,
                    format!("{:.0}%", self.progress * 100.0),
                )
            });
        }

        response
    }
}

impl CircularProgressBar {
    fn paint_at(&self, ui: &Ui, rect: Rect) {
        let visuals = &ui.visuals().widgets.inactive;
        let painter = ui.painter_at(rect);

        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.5 - 2.0;
        let stroke_width = (radius * 0.1).max(2.0).min(4.0);

        // Background circle
        painter.circle_stroke(
            center,
            radius,
            Stroke::new(stroke_width * 0.5, visuals.bg_stroke.color),
        );

        // Progress calculation
        let (start_angle, end_angle) = if self.animate {
            let time = ui.input(|i| i.time) as f32;
            let rotation = time * 2.0; // Speed of rotation
            let arc_length = TAU * 0.25; // Quarter circle
            let start = rotation % TAU - FRAC_PI_2;
            (start, start + arc_length)
        } else {
            let start_angle = -FRAC_PI_2; // Start at top (12 o'clock)
            let progress_angle = TAU * self.progress;
            (start_angle, start_angle + progress_angle)
        };

        // Progress arc
        if self.progress > 0.0 || self.animate {
            let progress_color = if self.animate {
                ui.visuals().selection.bg_fill
            } else {
                // Interpolate between stroke color and selection color
                let from = visuals.fg_stroke.color;
                let to = ui.visuals().selection.bg_fill;
                Color32::from_rgba_premultiplied(
                    (from.r() as f32 + (to.r() as f32 - from.r() as f32) * self.progress) as u8,
                    (from.g() as f32 + (to.g() as f32 - from.g() as f32) * self.progress) as u8,
                    (from.b() as f32 + (to.b() as f32 - from.b() as f32) * self.progress) as u8,
                    (from.a() as f32 + (to.a() as f32 - from.a() as f32) * self.progress) as u8,
                )
            };

            // Draw arc using path
            let mut points = Vec::new();
            let num_segments = ((end_angle - start_angle).abs() * radius / 2.0).ceil() as usize;
            let num_segments = num_segments.max(4);

            for i in 0..=num_segments {
                let angle = start_angle + (end_angle - start_angle) * (i as f32 / num_segments as f32);
                let x = center.x + radius * angle.cos();
                let y = center.y + radius * angle.sin();
                points.push(pos2(x, y));
            }

            for i in 0..points.len() - 1 {
                painter.line_segment(
                    [points[i], points[i + 1]],
                    Stroke::new(stroke_width, progress_color),
                );
            }
        }

        // Center text
        if let Some(text) = &self.text {
            let text_color = ui.visuals().text_color();
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }

        // Request repaint for animation
        if self.animate {
            ui.ctx().request_repaint();
        }
    }
}

/// Extension trait for Ui to add circular progress bar methods
pub trait CircularProgressBarExt {
    /// Add a circular progress bar
    fn circular_progress_bar(&mut self, progress: f32) -> Response;
    /// Add a circular progress bar with custom size
    fn circular_progress_bar_with_size(&mut self, progress: f32, size: f32) -> Response;
    /// Add an indeterminate circular progress bar (animated)
    fn circular_progress_bar_indeterminate(&mut self) -> Response;
}

impl CircularProgressBarExt for Ui {
    fn circular_progress_bar(&mut self, progress: f32) -> Response {
        self.add(CircularProgressBar::new(progress))
    }

    fn circular_progress_bar_with_size(&mut self, progress: f32, size: f32) -> Response {
        self.add(CircularProgressBar::new(progress).size(size))
    }

    fn circular_progress_bar_indeterminate(&mut self) -> Response {
        self.add(CircularProgressBar::indeterminate())
    }
}