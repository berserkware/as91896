use iced::{Element, Color};
use iced::widget::{row, Column, Row, Text, column};
use crate::app::Message;

pub fn table(
    column_names: Vec<String>,
    row_data: Vec<Vec<String>>
) -> Element<'static, Message> {
    let mut columns = Row::new().spacing(20);

    for (i, name) in column_names.iter().enumerate() {
	let mut column = Column::new().spacing(5);
	column = column.push(Text::new(name.clone()));
	for data in row_data.clone() {
	    column = column.push(
		Text::new(
		    data[i].clone()).color(Color::from_rgba(255.0, 255.0, 255.0, 0.5)
		)
	    );
	}
	columns = columns.push(column);
    }

    columns.into()
}

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
