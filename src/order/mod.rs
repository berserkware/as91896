pub mod table;
pub mod form;

use rusqlite::{Connection, Error};
use chrono::NaiveDate;
use rand::prelude::*;

pub use self::form::OrderForm;

/// Represents an order in the database
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub receipt_number: i64,
    pub item_hired: String,
    pub how_many: i32,
    pub hired_on: NaiveDate,
    pub return_on: NaiveDate,
    pub boxes_needed: i32,
    pub raffle_number: i32,
}

impl Order {
    /// Creates the database table for Order in the given database if it doesn't exist.
    pub fn init_table(connection: &Connection) -> Result<usize, Error> {
	connection.execute(
	    "CREATE TABLE IF NOT EXISTS CustomerOrder (
                id             INTEGER PRIMARY KEY,
                customer_name  TEXT NOT NULL,
                receipt_number INTEGER NOT NULL,
                item_hired     TEXT NOT NULL,
                how_many       INTEGER NOT NULL,
                hired_on       TEXT NOT NULL,
                return_on      TEXT NOT NULL,
                boxes_needed   INTEGER NOT NULL,
                raffle_number  INTEGER NOT NULL
            )",
	    (),
	)
    }

    /// Creates a new Order in the database.
    pub fn new(
	connection: &Connection,
	customer_name: String,
	receipt_number: i64,
	item_hired: String,
	how_many: i32,
	hired_on: NaiveDate,
	return_on: NaiveDate,
    ) -> Result<Self, Error> {
	let boxes_needed = boxes_needed(how_many);

	let mut rng = rand::rng();
	let raffle_number = rng.random_range(0..=1000);
	
	connection.execute(
	    "INSERT INTO CustomerOrder (
                customer_name, 
                receipt_number, 
                item_hired,   
                how_many,       
                hired_on,
                return_on,  
                boxes_needed,
                raffle_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
	    [
		customer_name,
		receipt_number.to_string(),
		item_hired,
		how_many.to_string(),
		hired_on.format("%Y-%m-%d").to_string(),
		return_on.format("%Y-%m-%d").to_string(),
		boxes_needed.to_string(),
		raffle_number.to_string(),
	    ],
	)?;

	Self::get_by_id(connection, connection.last_insert_rowid() as i32)
    }

    /// Retrieves an order from the database by its id.
    pub fn get_by_id(connection: &Connection, id: i32) -> Result<Self, Error> {
	let mut stmt = connection.prepare(
	    "SELECT * FROM CustomerOrder WHERE id = ?1;"
	)?;

	stmt.query_row([id], |row| {
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
		raffle_number: row.get(8).unwrap(),
	    })
	})
    }

    /// Gets all the orders in the database.
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
		raffle_number: row.get(8).unwrap(),
	    })
	}).unwrap().map(|o| o.unwrap()).collect()
    }

    /// Deletes an order from the database.
    pub fn delete(self, connection: &Connection) -> Result<usize, Error> {
	connection.execute(
	    "DELETE FROM CustomerOrder WHERE id = ?1", [self.id]
	)
    }
}

/// Gets how many boxes needed to store the given amount of items
fn boxes_needed(items: i32) -> i32 {
    (items + 25 - 1) / 25
}

#[cfg(test)]
mod tests {
    use crate::database::init_db_tables;

    use super::*;

    #[test]
    fn test_init_order_table() {
	let con = Connection::open_in_memory().unwrap();

	assert!(Order::init_table(&con).is_ok());
    }

    #[test]
    fn test_new_order() {
	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);

	assert!(Order::new(
	    &con,
	    "Test Person".to_string(),
	    15,
	    "Test Item".to_string(),
	    26,
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	).is_ok());
    }

    #[test]
    fn test_boxes_needed() {
	assert_eq!(boxes_needed(26), 2);
    }

    #[test]
    fn test_boxes_needed_zero() {
	assert_eq!(boxes_needed(0), 0);
    }

    #[test]
    fn test_boxes_needed_exact() {
	assert_eq!(boxes_needed(30), 2);
    }

    #[test]
    fn test_get_order_by_id() {
	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);

	let order = Order::new(
	    &con,
	    "Test Person".to_string(),
	    15,
	    "Test Item".to_string(),
	    26,
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	).unwrap();

	assert!(Order::get_by_id(&con, order.id).is_ok());
    }

    #[test]
    fn test_get_order_by_id_doesnt_exist() {
	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);
	
	assert!(Order::get_by_id(&con, 1132).is_err());
    }

    #[test]
    fn test_order_get_all() {
	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);

	let order = Order::new(
	    &con,
	    "Test Person".to_string(),
	    15,
	    "Test Item".to_string(),
	    26,
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	).unwrap();

	let orders = Order::get_all(&con);
	
	assert!(!orders.is_empty());
	assert_eq!(orders[0], order);
    }

    #[test]
    fn test_order_delete() {
	let con = Connection::open_in_memory().unwrap();

	init_db_tables(&con);

	let order = Order::new(
	    &con,
	    "Test Person".to_string(),
	    15,
	    "Test Item".to_string(),
	    26,
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	    NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
	).unwrap();

	assert!(order.delete(&con).is_ok());
    }
}
