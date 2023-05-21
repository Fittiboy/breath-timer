#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use std::time::{Duration, Instant};

use egui::{hex_color, vec2, CentralPanel, Context, ProgressBar};

const PER_FULL_BREATH: f32 = 60000. / 5.5;
const PER_HALF_BREATH: f32 = PER_FULL_BREATH / 2.;

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
            let millis_passed = self.start.elapsed().as_millis();
            let breaths_completed = millis_passed as f32 / PER_FULL_BREATH;
            let progress = match self.start.elapsed().as_millis() as f32 % PER_FULL_BREATH {
                elapsed if elapsed < PER_HALF_BREATH => elapsed / PER_HALF_BREATH,
                elapsed => (PER_FULL_BREATH - elapsed) / PER_HALF_BREATH,
            };
            ui.add(ProgressBar::new(progress).fill(hex_color!("#2dbfb8")));
            ui.label(format!("Breaths completed: {}", breaths_completed as u32));
            let seconds_passed = self.start.elapsed().as_secs();
            let (minutes, seconds) = (seconds_passed / 60, seconds_passed % 60);
            let (hours, minutes) = (minutes / 60, minutes % 60);
            let time_passed = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
            ui.label(time_passed);
        });
    }
}
