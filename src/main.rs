use eframe::egui::{self, vec2, ViewportBuilder};
use std::time::{Duration, Instant};

struct TimerApp {
    time: String,
    ball_x: f32,
    direction: f32,
    last_update: Instant,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            time: String::new(),
            ball_x: 0.0,
            direction: 1.0,
            last_update: Instant::now(),
        }
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update);
        self.last_update = now;

        // Update time
        let current_time = chrono::Local::now();
        self.time = current_time.format("%H:%M:%S%.3f").to_string();

        // Animate ball
        self.ball_x += self.direction * 100.0 * delta.as_secs_f32();
        if self.ball_x <= 0.0 || self.ball_x >= 980.0 {
            self.direction *= -1.0;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Create a custom RichText with larger size and custom color
            let time_text = egui::RichText::new(&self.time)
                .size(200.0)
                .color(egui::Color32::from_rgb(255, 255, 255));
            // Add the custom text to the UI
            ui.add(egui::Label::new(time_text));
            
            let (response, painter) = ui.allocate_painter(egui::Vec2::new(1000.0, 100.0), egui::Sense::hover());
            let rect = response.rect;
            
            painter.circle(
                egui::Pos2::new(rect.left() + self.ball_x + 10.0, rect.top() + 50.0),
                50.0,
                egui::Color32::RED,
                egui::Stroke::none(),
            );
        });

        // Request a repaint
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder { 
            inner_size: Some(vec2(1000.0, 1000.0)),
            ..Default::default()},
        ..Default::default()
    };
    eframe::run_native(
        "Timer with Animation",
        options,
        Box::new(|_cc| Box::new(TimerApp::default())),
    )
}