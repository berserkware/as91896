use iced::{Element, Color};
use iced::widget::{row, Column, Row, Text, column};
use crate::app::Message;

pub fn field_error(error: Option<String>) -> Element<'static, Message> {
    column![
	Text::new(error.unwrap_or("".to_string())).color(Color::from_rgb(255.0, 0.0, 0.0))
    ].into()
}

pub fn required_input_label(text: &str) -> Element<'static, Message> {
    row![
	Text::new(text.to_string()),
	Text::new(" *").color(Color::from_rgb(255.0, 0.0, 0.0)),
    ].into()
}
