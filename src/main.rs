use framework::{AppConfig, AppState, run};
use log::{LevelFilter, info};

mod framework;
mod graphics;

#[allow(dead_code)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/shader.rs"));
}

#[derive(Default)]
struct MyAppState {
    green_value: f32,
}

impl AppState for MyAppState {
    fn init(&mut self) {
        self.green_value = 0.0;
    }

    fn frame(&mut self, _delta_time: f32) -> sokol::gfx::Color {
        self.green_value += 0.01;
        if self.green_value > 1.0 {
            self.green_value = 0.0;
        }

        sokol::gfx::Color {
            r: 1.0,
            g: self.green_value,
            b: 0.0,
            a: 1.0,
        }
    }

    fn cleanup(&mut self) {
        info!("cleaning up");
    }
}

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(LevelFilter::Debug)
        .init();

    let config = AppConfig {
        window_title: "vatt".to_string(),
        width: 800,
        height: 600,
        sample_count: 4,
    };

    run(config, MyAppState::default());
}
