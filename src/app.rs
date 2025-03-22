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
use iced::{Color, Element, Length, Task, Theme};

use iced_aw::widget::{Tabs, TabLabel};
use iced_table::table;
use rusqlite::{Connection, Result};

use crate::order::{Order, OrderBuilder};
use crate::helpers::{field_error, required_input_label};
use crate::order_table::{OrderColumn, OrderColumnKind};

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
    DeleteOrder(i32),

    SyncOrderTableHeader(scrollable::AbsoluteOffset),
    OrderTableResizing(usize, f32),
    OrderTableResized,
    
}

pub struct App {
    db_connection: Connection,
    
    active_tab: TabId,
    
    orders: Vec<Order>,
    order_builder: OrderBuilder,
    
    order_table_header: scrollable::Id,
    order_table_body: scrollable::Id,
    order_table_columns: Vec<OrderColumn>,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
	let db_connection = Self::init_db();
	let orders = Order::get_all(&db_connection);
	
	let app = Self {
            db_connection,
            active_tab: TabId::Orders,
	    orders,
	    order_builder: OrderBuilder::default(),
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
	    ]
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
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
	    },
	    Message::DeleteOrder(id) => {
		let order = Order::get_by_id(&self.db_connection, id).unwrap();
		order.delete(&self.db_connection).unwrap();
		self.orders = Order::get_all(&self.db_connection)
	    },
	    Message::SyncOrderTableHeader(offset) => {
                return Task::batch(vec![
                    scrollable::scroll_to(self.order_table_header.clone(), offset),
                ])
            }
            Message::OrderTableResizing(index, offset) => {
                if let Some(column) = self.order_table_columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            Message::OrderTableResized => self.order_table_columns.iter_mut().for_each(|column| {
                if let Some(offset) = column.resize_offset.take() {
                    column.width += offset;
                }
            }),
        }

	Task::none()
    }

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
