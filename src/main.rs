mod app;
mod order;
mod helpers;
mod database;

use crate::app::App;
use crate::database::init_db;
use iced::{window::Settings, Result};

fn main() -> Result {
    iced::application(App::title, App::update, App::view)
        .window(Settings {
            size: iced::Size::new(1200.0, 600.0),
	    min_size: Some(iced::Size::new(1200.0, 600.0)),
            ..Default::default()
        })
        .theme(App::theme)
        .run_with(|| App::new(init_db()))
}
