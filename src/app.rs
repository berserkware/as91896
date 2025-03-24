use iced::widget::{
    button,
    center,
    column,
    scrollable,
    text_input,
    text,
    responsive,
    row,
    container,
};
use iced::{Element, Length, Task, Theme};

use iced_aw::widget::{Tabs, TabLabel};
use iced_table::table;
use rusqlite::Connection;

use crate::order::{Order, OrderForm};
use crate::order::table::{OrderColumn, OrderColumnKind};
use crate::helpers::{field_error, required_input_label};

/// Used to represent the current tab the program is on.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TabId {
    Orders,
    AddOrder,
    Raffle,
}

/// Represents an event sent by the UI to the app
#[derive(Clone, Debug)]
pub enum Message {
    TabSelected(TabId),
    CustomerNameChanged(String),
    ReceiptNumberChanged(String),
    ItemHiredChanged(String),
    HowManyChanged(String),
    HiredOnChanged(String),
    ReturnOnChanged(String),
    AddOrder,
    DeleteOrder(i32),

    SyncOrderTableHeader(scrollable::AbsoluteOffset),
    OrderTableResizing(usize, f32),
    OrderTableResized,

    SyncRaffleTableHeader(scrollable::AbsoluteOffset),
    RaffleTableResizing(usize, f32),
    RaffleTableResized,
}

/// Stores the state and methods of the app
pub struct App {
    // The database connection
    db_connection: Connection,

    // What tab is active
    active_tab: TabId,

    // The orders in the table
    orders: Vec<Order>,

    // The form for the add order screen
    order_form: OrderForm,

    // Infomation for the order table
    order_table_header: scrollable::Id,
    order_table_body: scrollable::Id,
    order_table_columns: Vec<OrderColumn>,

    // Infomation for the raffle table
    raffle_table_header: scrollable::Id,
    raffle_table_body: scrollable::Id,
    raffle_table_columns: Vec<OrderColumn>,
}

impl App {
    /// Creates the app and inits the database.
    pub fn new(db_connection: Connection) -> (Self, Task<Message>) {
	let orders = Order::get_all(&db_connection);
	
	let app = Self {
            db_connection,
            active_tab: TabId::Orders,
	    orders,
	    order_form: OrderForm::default(),

	    order_table_header: scrollable::Id::unique(),
	    order_table_body: scrollable::Id::unique(),
	    order_table_columns: vec![
		OrderColumn::new(OrderColumnKind::CustomerName),
		OrderColumn::new(OrderColumnKind::RecieptNumber),
		OrderColumn::new(OrderColumnKind::ItemHired),
		OrderColumn::new(OrderColumnKind::HowMany),
		OrderColumn::new(OrderColumnKind::HiredOn),
		OrderColumn::new(OrderColumnKind::ReturnOn),
		OrderColumn::new(OrderColumnKind::BoxesNeeded),
		OrderColumn::new(OrderColumnKind::Delete),
	    ],

	    raffle_table_header: scrollable::Id::unique(),
	    raffle_table_body: scrollable::Id::unique(),
	    raffle_table_columns: vec![
		OrderColumn::new(OrderColumnKind::CustomerName),
		OrderColumn::new(OrderColumnKind::RaffleNumber),
	    ],
        };
	
        (app,Task::none())
    }

    /// Gets the title of the app.
    pub fn title(&self) -> String {
        return "Order Management Application".into();
    }

    /// Gets the theme of the app.
    pub fn theme(&self) -> Theme {
	Theme::Dark
    }

    /// Responds to events from the UI.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
	    Message::TabSelected(tab) => {
		self.active_tab = tab;
		self.order_form = OrderForm::default();
	    },
	    Message::CustomerNameChanged(customer_name) => {
		self.order_form.customer_name = customer_name;
		self.order_form.customer_name_show_error = true;
	    },
	    Message::ReceiptNumberChanged(receipt_number) => {
		self.order_form.receipt_number = receipt_number;
		self.order_form.receipt_number_show_error = true;
	    },
	    Message::ItemHiredChanged(item_hired) => {
		self.order_form.item_hired = item_hired;
		self.order_form.item_hired_show_error = true;
	    },
	    Message::HowManyChanged(how_many) => {
		self.order_form.how_many = how_many;
		self.order_form.how_many_show_error = true;
	    },
	    Message::HiredOnChanged(hired_on) => {
		self.order_form.hired_on = hired_on;
		self.order_form.hired_on_show_error = true;
	    },
	    Message::ReturnOnChanged(return_on) => {
		self.order_form.return_on = return_on;
		self.order_form.return_on_show_error = true;
	    },
	    Message::AddOrder => {
		match self.order_form.create_order(&self.db_connection) {
		    Ok(order) => {
			self.orders.push(order);
			self.active_tab = TabId::Orders;
		    },
		    Err(_) => {
			self.order_form.customer_name_show_error = true;
			self.order_form.receipt_number_show_error = true;
			self.order_form.item_hired_show_error = true;
			self.order_form.how_many_show_error = true;
			self.order_form.hired_on_show_error = true;
			self.order_form.return_on_show_error = true;
		    },
		}
	    },
	    Message::DeleteOrder(id) => {
		let order = Order::get_by_id(&self.db_connection, id).unwrap();
		order.delete(&self.db_connection).unwrap();

		// Refresh the order list to not show the deleted order
		self.orders = Order::get_all(&self.db_connection)
	    },
	    Message::SyncOrderTableHeader(offset) => {
		// Return background task to synch the order table header
                return Task::batch(vec![
                    scrollable::scroll_to(self.order_table_header.clone(), offset),
                ])
            }
            Message::OrderTableResizing(index, offset) => {
		// Updates the resize offset for a specific order table column
                if let Some(column) = self.order_table_columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            Message::OrderTableResized => {
		// Applies the stored resize offsets to update column widths in the order table
		self.order_table_columns.iter_mut().for_each(|column| {
                    if let Some(offset) = column.resize_offset.take() {
			column.width += offset;
                    }
		})
	    },
	    Message::SyncRaffleTableHeader(offset) => {
		// Returns a background task to sync the raffle table header position
                return Task::batch(vec![
                    scrollable::scroll_to(self.raffle_table_header.clone(), offset),
                ])
            }
            Message::RaffleTableResizing(index, offset) => {
		// Updates the resize offset for a specific raffle table column
                if let Some(column) = self.raffle_table_columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            Message::RaffleTableResized => {
		// Applies the stored resize offsets to update column widths in the raffle table
		self.raffle_table_columns.iter_mut().for_each(|column| {
                    if let Some(offset) = column.resize_offset.take() {
			column.width += offset;
                    }
		})
	    },
        }

	Task::none()
    }

    /// Produces the UI tree of the application.
    pub fn view(&self) -> Element<Message> {
	Tabs::new(Message::TabSelected)
	    .push(
		TabId::Orders,
		TabLabel::Text("Orders".to_string()),
		column![
		    container(
			text("Orders").size(30)
		    ).padding(10),
		    responsive(|size| {
			table(
			    self.order_table_header.clone(),
			    self.order_table_body.clone(),
			    &self.order_table_columns,
			    &self.orders,
			    Message::SyncOrderTableHeader,
			).on_column_resize(
			    Message::OrderTableResizing,
			    Message::OrderTableResized
			).min_width(
			    size.width
			).into()
		    })
		],
	    )
	    .push(
		TabId::AddOrder,
		TabLabel::Text("Add Order".to_string()),
		center(scrollable(column![
		    text("Add Order").size(30),
		    column![
			required_input_label("Customer Name"),
			text_input("", &self.order_form.customer_name)
			    .on_input(Message::CustomerNameChanged),
			field_error(self.order_form.get_visible_field_error("customer_name")),
		    ],
		    column![
			required_input_label("Receipt Number"),
			text_input("", &self.order_form.receipt_number)
			    .on_input(Message::ReceiptNumberChanged),
			field_error(self.order_form.get_visible_field_error("receipt_number")),
		    ],
		    column![
			required_input_label("Item Hired"),
			text_input("", &self.order_form.item_hired)
			    .on_input(Message::ItemHiredChanged),
			field_error(self.order_form.get_visible_field_error("item_hired")),
		    ],
		    column![
			required_input_label("How Many"),
			text_input("", &self.order_form.how_many)
			    .on_input(Message::HowManyChanged),
			field_error(self.order_form.get_visible_field_error("how_many")),
		    ],
		    row![
			column![
			    required_input_label("Hired On"),
			    text_input("YYYY-mm-dd", &self.order_form.hired_on)
				.on_input(Message::HiredOnChanged),
			    field_error(self.order_form.get_visible_field_error("hired_on")),
			],
			column![
			    required_input_label("Return On"),
			    text_input("YYYY-mm-dd", &self.order_form.return_on)
				.on_input(Message::ReturnOnChanged),
			    field_error(self.order_form.get_visible_field_error("return_on")),
			],
		    ].spacing(20),
		    button("Add").on_press(Message::AddOrder),
		].padding([10, 0]).width(Length::Fixed(500.0)).spacing(10)))
	    )
	    .push(
		TabId::Raffle,
		TabLabel::Text("Raffle".to_string()),
		column![
		    container(
			text("Raffle").size(30)
		    ).padding(10),
		    responsive(|size| {
			table(
			    self.raffle_table_header.clone(),
			    self.raffle_table_body.clone(),
			    &self.raffle_table_columns,
			    &self.orders,
			    Message::SyncRaffleTableHeader,
			).on_column_resize(
			    Message::RaffleTableResizing,
			    Message::RaffleTableResized
			).min_width(
			    size.width
			).into()
		    })
		],
	    )
	    .set_active_tab(&self.active_tab)
	    .into()
    }
}

#[cfg(test)]
mod test {
    use crate::database::init_db_tables;

    use super::*;

    #[test]
    fn test_app_tab_selected_message() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::TabSelected(TabId::AddOrder));
	assert_eq!(app.active_tab, TabId::AddOrder);
    }

    #[test]
    fn test_app_customer_name_changed() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::CustomerNameChanged("Test".to_string()));
	assert_eq!(app.order_form.customer_name, "Test".to_string());
	assert_eq!(app.order_form.customer_name_show_error, true);
    }

    #[test]
    fn test_app_receipt_number_changed() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::ReceiptNumberChanged("Test".to_string()));
	assert_eq!(app.order_form.receipt_number, "Test".to_string());
	assert_eq!(app.order_form.receipt_number_show_error, true);
    }

    #[test]
    fn test_app_item_hired_changed() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::ItemHiredChanged("Test".to_string()));
	assert_eq!(app.order_form.item_hired, "Test".to_string());
	assert_eq!(app.order_form.item_hired_show_error, true);
    }

    #[test]
    fn test_app_how_many_changed() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::HowManyChanged("Test".to_string()));
	assert_eq!(app.order_form.how_many, "Test".to_string());
	assert_eq!(app.order_form.how_many_show_error, true);
    }

    #[test]
    fn test_app_hired_on_changed() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::HiredOnChanged("Test".to_string()));
	assert_eq!(app.order_form.hired_on, "Test".to_string());
	assert_eq!(app.order_form.hired_on_show_error, true);
    }

    #[test]
    fn test_app_return_on_changed() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::ReturnOnChanged("Test".to_string()));
	assert_eq!(app.order_form.return_on, "Test".to_string());
	assert_eq!(app.order_form.return_on_show_error, true);
    }

    #[test]
    fn test_app_add_order() {
	let connection = Connection::open_in_memory().unwrap();

	init_db_tables(&connection);
	
	let mut app = App::new(connection).0;

	let _ = app.update(Message::AddOrder);
	assert_eq!(app.order_form.customer_name_show_error, true);
	assert_eq!(app.order_form.receipt_number_show_error, true);
	assert_eq!(app.order_form.item_hired_show_error, true);
	assert_eq!(app.order_form.how_many_show_error, true);
	assert_eq!(app.order_form.hired_on_show_error, true);
	assert_eq!(app.order_form.return_on_show_error, true);
    }
}
