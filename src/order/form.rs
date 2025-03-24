use crate::order::Order;
use rusqlite::Connection;
use chrono::NaiveDate;

/// Represents a form to create an Order
#[derive(Default)]
pub struct OrderForm {
    pub customer_name: String,
    pub customer_name_show_error: bool,
    pub receipt_number: String,
    pub receipt_number_show_error: bool,
    pub item_hired: String,
    pub item_hired_show_error: bool,
    pub how_many: String,
    pub how_many_show_error: bool,
    pub hired_on: String,
    pub hired_on_show_error: bool,
    pub return_on: String,
    pub return_on_show_error: bool,
}

impl OrderForm {
    /// Gets customer_name, or Err if it is invalid.
    fn get_valid_customer_name(&self) -> Result<String, String> {
	if self.customer_name.is_empty() {
            return Err("Customer name is required".to_string());
	} else {
            if self.customer_name.len() > 30 {
		return Err("Customer name must be less than 30 characters".to_string());
            } else if self.customer_name.len() < 3 {
		return Err("Customer name must be at least 3 characters".to_string());
            }
	}

	Ok(self.customer_name.clone())
    }

    /// Gets reciept_number as an integer, or Err if it is invalid.
    fn get_valid_receipt_number(&self) -> Result<i64, String> {
	if self.receipt_number.is_empty() {
            return Err("Receipt number is required".to_string());
	} else {
	    match self.receipt_number.parse::<i64>() {
		Err(_) => {
		    return Err("Receipt number must be a 64-bit integer".to_string());
		}
		Ok(rn) => {
		    return Ok(rn);
		}
	    }
	}
    }

    /// Gets item_hired, or Err if it is invalid.
    fn get_valid_item_hired(&self) -> Result<String, String> {
	if self.item_hired.is_empty() {
            return Err("Item hired is required".to_string());
	} else {
            if self.item_hired.len() > 30 {
		return Err("Item hired must be less than 30 characters".to_string());
            } else if self.item_hired.len() < 3 {
		return Err("Item hired must be at least 3 characters".to_string());
            }
	}

	Ok(self.item_hired.clone())
    }

    /// Gets how_many as an integer, or Err if it is invalid.
    fn get_valid_how_many(&self) -> Result<i32, String> {
	if self.how_many.is_empty() {
            return Err("How many is required".to_string());
	} else {
            match self.how_many.parse::<i32>() {
		Ok(num) => {
                    if num < 1 {
			return Err("How many must be at least 1".to_string());
                    } else if num > 500 {
			return Err("How many must not be more than 500".to_string());
                    }

		    return Ok(num);
		},
		Err(_) => {
                    return Err("How many must be an integer".to_string());
		},
            }
	}
    }

    /// Gets hired_on as a NaiveDate, or Err if it is invalid.
    fn get_valid_hired_on(&self) -> Result<NaiveDate, String> {
	if self.hired_on.is_empty() {
            return Err("Hired on date is required".to_string());
	}
	else {
	    match NaiveDate::parse_from_str(self.hired_on.as_str(), "%Y-%m-%d") {
		Ok(ho) => {
		    return Ok(ho);
		},
		Err(_) => {
		    return Err("Hired on date must be formatted as YYYY-MM-DD e.g. 2025-03-18".to_string());
		}
	    }
	}
    }

    /// Gets return_on as a NaiveDate, or Err if it is invalid.
    fn get_valid_return_on(&self) -> Result<NaiveDate, String> {
	if self.return_on.is_empty() {
            return Err("Return on date is required".to_string());
	}
	else {
	    match NaiveDate::parse_from_str(self.return_on.as_str(), "%Y-%m-%d") {
		Ok(ro) => {
		    return Ok(ro);
		},
		Err(_) => {
		    return Err("Return on date must be formatted as YYYY-MM-DD e.g. 2025-03-18".to_string());
		}
	    }
	}
    }

    /// Gets the error for a given field if its corresponding *_show_error is set to true.
    pub fn get_visible_field_error(&self, field: &str) -> Option<String> {
	match field {
            "customer_name" => {
		if self.customer_name_show_error {
		    if let Err(e) = self.get_valid_customer_name() {
			return Some(e);
		    }
		}
            },
            "receipt_number" => {
		if self.receipt_number_show_error {
		    if let Err(e) = self.get_valid_receipt_number() {
			return Some(e);
		    }
		}
            },
	    "item_hired" => {
		if self.item_hired_show_error {
		    if let Err(e) = self.get_valid_item_hired() {
			return Some(e);
		    }
		}
            },
	    "how_many" => {
		if self.how_many_show_error {
		    if let Err(e) = self.get_valid_how_many() {
			return Some(e);
		    }
		}
            },
	    "hired_on" => {
		if self.hired_on_show_error {
		    if let Err(e) = self.get_valid_hired_on() {
			return Some(e);
		    }
		}
            },
	    "return_on" => {
		if self.return_on_show_error {
		    if let Err(e) = self.get_valid_return_on() {
			return Some(e);
		    }
		}
            },
            _ => (),
	};

	None
    }

    /// Creates an Order if all fields of the form are valid.
    pub fn create_order(&self, connection: &Connection) -> Result<Order, String> {
	Ok(
	    Order::new(
		connection,
		self.get_valid_customer_name()?,
		self.get_valid_receipt_number()?,
		self.get_valid_item_hired()?,
		self.get_valid_how_many()?,
		self.get_valid_hired_on()?,
		self.get_valid_return_on()?,
	    ).unwrap()
	)
    }
}

#[cfg(test)]
mod test {
    use crate::database::init_db_tables;

    use super::*;

    #[test]
    fn test_get_valid_customer_name() {
	let mut form = OrderForm::default();
	form.customer_name = "Testing".to_string();
	
	assert!(form.get_valid_customer_name().is_ok());
    }
    
    #[test]
    fn test_get_valid_customer_name_empty() {
	let form = OrderForm::default();

	assert!(form.get_valid_customer_name().is_err());
    }

    #[test]
    fn test_get_valid_customer_name_too_short() {
	let mut form = OrderForm::default();
	form.customer_name = "aa".to_string();
	
	assert!(form.get_valid_customer_name().is_err());
    }

    #[test]
    fn test_get_valid_customer_name_too_long() {
	let mut form = OrderForm::default();
	form.customer_name = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();

	assert!(form.get_valid_customer_name().is_err());
    }

    #[test]
    fn test_get_valid_receipt_number() {
	let mut form = OrderForm::default();
	form.receipt_number = "123".to_string();

	assert!(form.get_valid_receipt_number().is_ok());
    }

    #[test]
    fn test_get_valid_receipt_number_empty() {
	let form = OrderForm::default();

	assert!(form.get_valid_receipt_number().is_err());
    }

    #[test]
    fn test_get_valid_receipt_number_not_a_number() {
	let mut form = OrderForm::default();
	form.receipt_number = "hello".to_string();

	assert!(form.get_valid_receipt_number().is_err());
    }

    #[test]
    fn test_get_valid_item_hired() {
	let mut form = OrderForm::default();
	form.item_hired = "Test Item".to_string();

	assert!(form.get_valid_item_hired().is_ok())
    }

    #[test]
    fn test_get_valid_item_hired_empty() {
	let form = OrderForm::default();

	assert!(form.get_valid_item_hired().is_err())
    }

    #[test]
    fn test_get_valid_item_hired_too_short() {
	let mut form = OrderForm::default();
	form.item_hired = "aa".to_string();

	assert!(form.get_valid_item_hired().is_err())
    }

    #[test]
    fn test_get_valid_item_hired_too_long() {
	let mut form = OrderForm::default();
	form.item_hired = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();

	assert!(form.get_valid_item_hired().is_err())
    }

    #[test]
    fn test_get_valid_how_many() {
	let mut form = OrderForm::default();
	form.how_many = "250".to_string();

	assert!(form.get_valid_how_many().is_ok());
    }

    #[test]
    fn test_get_valid_how_many_empty() {
	let form = OrderForm::default();

	assert!(form.get_valid_how_many().is_err());
    }

    #[test]
    fn test_get_valid_how_many_not_an_int() {
	let mut form = OrderForm::default();
	form.how_many = "asdf".to_string();

	assert!(form.get_valid_how_many().is_err());
    }

    #[test]
    fn test_get_valid_how_many_too_small() {
	let mut form = OrderForm::default();
	form.how_many = "-24".to_string();

	assert!(form.get_valid_how_many().is_err());
    }

    #[test]
    fn test_get_valid_how_many_too_big() {
	let mut form = OrderForm::default();
	form.how_many = "21442".to_string();

	assert!(form.get_valid_how_many().is_err());
    }

    #[test]
    fn test_get_valid_hired_on() {
	let mut form = OrderForm::default();
	form.hired_on = "2024-03-24".to_string();

	assert!(form.get_valid_hired_on().is_ok());
    }

    #[test]
    fn test_get_valid_hired_on_empty() {
	let form = OrderForm::default();

	assert!(form.get_valid_hired_on().is_err());
    }

    #[test]
    fn test_get_valid_hired_on_invalid_format() {
	let mut form = OrderForm::default();
	form.hired_on = "2024/03-asdf".to_string();

	assert!(form.get_valid_hired_on().is_err());
    }

    #[test]
    fn test_get_valid_return_on() {
	let mut form = OrderForm::default();
	form.return_on = "2024-03-24".to_string();

	assert!(form.get_valid_return_on().is_ok());
    }

    #[test]
    fn test_get_valid_return_on_empty() {
	let form = OrderForm::default();

	assert!(form.get_valid_return_on().is_err());
    }

    #[test]
    fn test_get_valid_return_on_invalid_format() {
	let mut form = OrderForm::default();
	form.return_on = "2024/03-asdf".to_string();

	assert!(form.get_valid_return_on().is_err());
    }

    #[test]
    fn test_form_create_order() {
	let mut form = OrderForm::default();
	form.customer_name = "Test".to_string();
	form.receipt_number = "123".to_string();
	form.item_hired = "Test Item".to_string();
	form.how_many = "123".to_string();
	form.hired_on = "1-1-1".to_string();
	form.return_on = "1-1-2".to_string();

	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);

	assert!(form.create_order(&con).is_ok());
    }

    #[test]
    fn test_form_create_order_invalid() {
	let mut form = OrderForm::default();
	form.customer_name = "Test".to_string();
	form.receipt_number = "12asdf3".to_string();
	form.item_hired = "Test Item".to_string();
	form.how_many = "123asdf".to_string();
	form.hired_on = "1-1-1".to_string();
	form.return_on = "1-1-2".to_string();

	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);

	assert!(form.create_order(&con).is_err());
    }
}
