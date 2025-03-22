use iced::{Element, Length, Theme, Renderer};
use iced::widget::{container, text, button};
use iced_table::table;
use crate::app::Message;
use crate::order::Order;

pub struct OrderColumn {
    pub kind: OrderColumnKind,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl OrderColumn {
    pub fn new(kind: OrderColumnKind) -> Self {
	let width = match kind {
	    OrderColumnKind::CustomerName => 400.0,
	    OrderColumnKind::RecieptNumber => 120.0,
	    OrderColumnKind::ItemHired => 150.0,
	    OrderColumnKind::HowMany => 100.0,
	    OrderColumnKind::HiredOn => 100.0,
	    OrderColumnKind::ReturnOn => 100.0,
	    OrderColumnKind::BoxesNeeded => 60.0,
	    OrderColumnKind::Delete => 100.0,
	};

	Self {
	    kind,
	    width,
	    resize_offset: None,
	}
    }
}

pub enum OrderColumnKind {
    CustomerName,
    RecieptNumber,
    ItemHired,
    HowMany,
    HiredOn,
    ReturnOn,
    BoxesNeeded,
    Delete,
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for OrderColumn {
    type Row = Order;
    
    fn header(&'a self, _col_size: usize) -> Element<'a, Message> {
	let content =  match self.kind {
	    OrderColumnKind::CustomerName => "Customer Name",
	    OrderColumnKind::RecieptNumber => "Reciept No.",
	    OrderColumnKind::ItemHired => "Item Hired",
	    OrderColumnKind::HowMany => "How Many",
	    OrderColumnKind::HiredOn => "Hired On",
	    OrderColumnKind::ReturnOn => "Return On",
	    OrderColumnKind::BoxesNeeded => "Boxes",
	    OrderColumnKind::Delete => "",
	};

	container(text(content)).center_y(24).into()
    }

    fn cell(&'a self, _col_index: usize, _row_index: usize, row: &'a Order) -> Element<'a, Message> {
	let content: Element<_> = match self.kind {
	    OrderColumnKind::CustomerName => text(row.customer_name.clone()).into(),
	    OrderColumnKind::RecieptNumber => text(row.receipt_number.to_string()).into(),
	    OrderColumnKind::ItemHired => text(row.item_hired.clone()).into(),
	    OrderColumnKind::HowMany => text(row.how_many.to_string()).into(),
	    OrderColumnKind::HiredOn => text(row.hired_on.to_string()).into(),
	    OrderColumnKind::ReturnOn => text(row.return_on.to_string()).into(),
	    OrderColumnKind::BoxesNeeded => text(row.boxes_needed.to_string()).into(),
	    OrderColumnKind::Delete => button(text("Delete"))
		.on_press(Message::DeleteOrder(row.id))
		.into(),
	};

	container(content).width(Length::Fill).center_y(32).into()
    }

    fn width(&self) -> f32 {
	self.width
    }

    fn resize_offset(&self) -> Option<f32> {
	self.resize_offset
    }
}
