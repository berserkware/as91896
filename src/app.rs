use iced::{
    widget::{button, center, column, horizontal_space, row, scrollable, text_input, Column, Text}, Color, Element, Length, Task, Theme
};
use iced_aw::widget::{Tabs, TabLabel};
use rusqlite::{Connection, Result};

use crate::order::{Order, OrderBuilder};
use crate::helpers::{table, field_error, required_input_label};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TabId {
    Orders,
    AddOrder,
}

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
    
}

pub struct App {
    db_connection: Connection,
    orders: Vec<Order>,
    active_tab: TabId,
    order_builder: OrderBuilder,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
	let db_connection = Self::init_db();
	let orders = Order::get_all(&db_connection);
	
	let app = Self {
            db_connection,
	    orders,
            active_tab: TabId::Orders,
	    order_builder: OrderBuilder::default(),
        };
	
        (app,Task::none())
    }

    fn init_db() -> Connection {
	let connection = Connection::open("./orders.db").unwrap();

	Order::init_table(&connection);

	connection
    }

    pub fn title(&self) -> String {
        return "as91869".into();
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn update(&mut self, message: Message) {
        match message {
	    Message::TabSelected(tab) => {
		self.active_tab = tab;
		self.order_builder = OrderBuilder::default();
	    },
	    Message::CustomerNameChanged(customer_name) => {
		self.order_builder.customer_name = customer_name;
		self.order_builder.customer_name_show_error = true;
	    },
	    Message::ReceiptNumberChanged(receipt_number) => {
		self.order_builder.receipt_number = receipt_number;
		self.order_builder.receipt_number_show_error = true;
	    },
	    Message::ItemHiredChanged(item_hired) => {
		self.order_builder.item_hired = item_hired;
		self.order_builder.item_hired_show_error = true;
	    },
	    Message::HowManyChanged(how_many) => {
		self.order_builder.how_many = how_many;
		self.order_builder.how_many_show_error = true;
	    },
	    Message::HiredOnChanged(hired_on) => {
		self.order_builder.hired_on = hired_on;
		self.order_builder.hired_on_show_error = true;
	    },
	    Message::ReturnOnChanged(return_on) => {
		self.order_builder.return_on = return_on;
		self.order_builder.return_on_show_error = true;
	    },
	    Message::AddOrder => {
		self.order_builder.customer_name_show_error = true;
		self.order_builder.receipt_number_show_error = true;
		self.order_builder.item_hired_show_error = true;
		self.order_builder.how_many_show_error = true;
		self.order_builder.hired_on_show_error = true;
		self.order_builder.return_on_show_error = true;

		match self.order_builder.create_order(&self.db_connection) {
		    Ok(order) => {
			self.orders.push(order);
			self.active_tab = TabId::Orders;
		    },
		    _ => (),
		}
	    }
        }
    }

    pub fn view(&self) -> Element<Message> {
	Tabs::new(Message::TabSelected)
	    .push(
		TabId::Orders,
		TabLabel::Text("Orders".to_string()),
		column![
		    Text::new("Orders").size(30),
		    scrollable(table(
			vec![
			    "Customer Name".to_string(),
			    "Receipt No.".to_string(),
			    "Item Hired".to_string(),
			    "How Many".to_string(),
			    "Hired On".to_string(),
			    "Return On".to_string(),
			    "Boxes Needed".to_string(),
			],
			self.orders.iter().map(
			    |order| vec![
				order.customer_name.clone(),
				order.receipt_number.to_string(),
				order.item_hired.clone(),
				order.how_many.to_string(),
				order.hired_on.to_string(),
				order.return_on.to_string(),
				order.boxes_needed.to_string(),
			    ]
			).collect()
		    ))
		].padding(10)
	    )
	    .push(
		TabId::AddOrder,
		TabLabel::Text("Add Order".to_string()),
		center(scrollable(column![
		    Text::new("Add Order").size(30),
		    column![
			required_input_label("Customer Name"),
			text_input("", &self.order_builder.customer_name)
			    .on_input(Message::CustomerNameChanged),
			field_error(self.order_builder.get_visible_field_error("customer_name")),
		    ],
		    column![
			required_input_label("Receipt Number"),
			text_input("", &self.order_builder.receipt_number)
			    .on_input(Message::ReceiptNumberChanged),
			field_error(self.order_builder.get_visible_field_error("receipt_number")),
		    ],
		    column![
			required_input_label("Item Hired"),
			text_input("", &self.order_builder.item_hired)
			    .on_input(Message::ItemHiredChanged),
			field_error(self.order_builder.get_visible_field_error("item_hired")),
		    ],
		    column![
			required_input_label("How Many"),
			text_input("", &self.order_builder.how_many)
			    .on_input(Message::HowManyChanged),
			field_error(self.order_builder.get_visible_field_error("how_many")),
		    ],
		    row![
			column![
			    required_input_label("Hired On"),
			    text_input("YYYY-mm-dd", &self.order_builder.hired_on)
				.on_input(Message::HiredOnChanged),
			    field_error(self.order_builder.get_visible_field_error("hired_on")),
			],
			column![
			    required_input_label("Return On"),
			    text_input("YYYY-mm-dd", &self.order_builder.return_on)
				.on_input(Message::ReturnOnChanged),
			    field_error(self.order_builder.get_visible_field_error("return_on")),
			],
		    ].spacing(20),
		    button("Add").on_press(Message::AddOrder),
		].padding([10, 0]).width(Length::Fixed(500.0)).spacing(10)))
	    )
	    .set_active_tab(&self.active_tab)
	    .into()
    }
}
