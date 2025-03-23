use iced::{Element, Color};
use iced::widget::{row, Text, column};
use crate::app::Message;

/// Returns a widget to represent an error in a form's field
pub fn field_error<'a>(error: Option<String>) -> Element<'a, Message> {
    column![
	Text::new(error.unwrap_or("".to_string())).color(Color::from_rgb(255.0, 0.0, 0.0))
    ].into()
}

/// Returns a widget to for a form input label
pub fn required_input_label<'a>(text: &str) -> Element<'a, Message> {
    row![
	Text::new(text.to_string()),
	Text::new(" *").color(Color::from_rgb(255.0, 0.0, 0.0)),
    ].into()
}
