#![allow(clippy::cast_precision_loss)]
use std::time::{Duration, Instant};

use egui::{hex_color, vec2, CentralPanel, Context, ProgressBar};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(300.0, 66.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Breath Timer",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
    .ok();
}

struct MyApp {
    start: std::time::Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint_after(Duration::from_millis(16));
            let seconds_passed = self.start.elapsed().as_secs();
            let breaths_completed = seconds_passed / 11;
            let progress = match (self.start.elapsed().as_millis() % 11000) as f32 {
                elapsed if elapsed < 5500. => elapsed / 5500.,
                elapsed => (11000. - elapsed) / 5500.,
            };
            ui.add(ProgressBar::new(progress).fill(hex_color!("#2dbfb8")));
            ui.label(format!("Breaths completed: {}", breaths_completed));
            let (minutes, seconds) = (seconds_passed / 60, seconds_passed % 60);
            let (hours, minutes) = (minutes / 60, minutes % 60);
            let time_passed = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
            ui.label(time_passed);
        });
    }
}
