mod app;
mod order;
mod helpers;

use crate::app::App;
use iced::{window::Settings, Result};

fn main() -> Result {
    iced::application(App::title, App::update, App::view)
        .window(Settings {
            size: iced::Size::new(1300.0, 600.0),
	    min_size: Some(iced::Size::new(1300.0, 600.0)),
            ..Default::default()
        })
        .theme(App::theme)
        .run_with(App::new)
}
