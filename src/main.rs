use api::response::Images;
use iced::{Application, Settings};

mod api;
mod gui;

fn main() {
    Images::run(Settings {
        window: iced::window::Settings {
            size: (780, 660),
            ..Default::default()
        },
        default_text_size: 18.0,
        antialiasing: true,
        ..Default::default()
    })
    .unwrap()
}
