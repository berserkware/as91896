use chrono::naive::NaiveDate;
use rusqlite::Connection;

#[derive(Default)]
pub struct OrderBuilder {
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

impl OrderBuilder {
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
		if self.hired_on_show_error {
		    if let Err(e) = self.get_valid_return_on() {
			return Some(e);
		    }
		}
            },
            _ => (),
	};

	None
    }

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

pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub receipt_number: i64,
    pub item_hired: String,
    pub how_many: i32,
    pub hired_on: NaiveDate,
    pub return_on: NaiveDate,
    pub boxes_needed: i32,
}

impl Order {
    pub fn init_table(connection: &Connection) {
	connection.execute(
	    "CREATE TABLE IF NOT EXISTS CustomerOrder (
                id             INTEGER PRIMARY KEY,
                customer_name  TEXT NOT NULL,
                receipt_number INTEGER NOT NULL,
                item_hired     TEXT NOT NULL,
                how_many       INTEGER NOT NULL,
                hired_on       TEXT NOT NULL,
                return_on      TEXT NOT NULL,
                boxes_needed   INTEGER NOT NULL
            )",
	    (),
	).unwrap();
    }

    pub fn new(
	connection: &Connection,
	customer_name: String,
	receipt_number: i64,
	item_hired: String,
	how_many: i32,
	hired_on: NaiveDate,
	return_on: NaiveDate,
    ) -> Option<Order> {
	let boxes_needed = (how_many + 25 - 1) / 25;
	
	let rows_affected = connection.execute(
	    "INSERT INTO CustomerOrder (
                customer_name, 
                receipt_number, 
                item_hired,   
                how_many,       
                hired_on,
                return_on,  
                boxes_needed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
	    [
		customer_name,
		receipt_number.to_string(),
		item_hired,
		how_many.to_string(),
		hired_on.format("%Y-%m-%d").to_string(),
		return_on.format("%Y-%m-%d").to_string(),
		boxes_needed.to_string(),
	    ],
	);

	if let Err(e) = rows_affected {
	    print!("HERE {:?}", e);
	} else if let Ok(ra) = rows_affected {
	    print!("Added!");
	}

	let mut stmt = connection.prepare(
	    "SELECT * FROM CustomerOrder WHERE id = last_insert_rowid();"
	).unwrap();

	let order = stmt.query_row([], |row| {
	    let hired_on: String = row.get(5).unwrap();
	    let return_on: String = row.get(6).unwrap();
	    
	    Ok(Self {
		id: row.get(0).unwrap(),
		customer_name: row.get(1).unwrap(),
		receipt_number: row.get(2).unwrap(),
		item_hired: row.get(3).unwrap(),
		how_many: row.get(4).unwrap(),
		hired_on: NaiveDate::parse_from_str(hired_on.as_str(), "%Y-%m-%d").unwrap(),
		return_on: NaiveDate::parse_from_str(return_on.as_str(), "%Y-%m-%d").unwrap(),
		boxes_needed: row.get(7).unwrap(),
	    })
	});

	match order {
	    Err(_) => {
		None
	    },
	    Ok(o) => {
		Some(o)
	    }
	}
    }

    pub fn get_all(connection: &Connection) -> Vec<Self> {
	let mut stmt = connection.prepare("SELECT * FROM CustomerOrder").unwrap();
	
	stmt.query_map([], |row| {
	    let hired_on: String = row.get(5).unwrap();
	    let return_on: String = row.get(6).unwrap();
	    
	    Ok(Self {
		id: row.get(0).unwrap(),
		customer_name: row.get(1).unwrap(),
		receipt_number: row.get(2).unwrap(),
		item_hired: row.get(3).unwrap(),
		how_many: row.get(4).unwrap(),
		hired_on: NaiveDate::parse_from_str(hired_on.as_str(), "%Y-%m-%d").unwrap(),
		return_on: NaiveDate::parse_from_str(return_on.as_str(), "%Y-%m-%d").unwrap(),
		boxes_needed: row.get(7).unwrap(),
	    })
	}).unwrap().map(|o| o.unwrap()).collect()
    }
}
