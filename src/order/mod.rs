pub mod table;
pub mod builder;

use rusqlite::{Connection, Error};
use chrono::NaiveDate;
use rand::prelude::*;

pub use self::builder::OrderBuilder;

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
                boxes_needed   INTEGER NOT NULL,
                raffle_number  INTEGER NOT NULL
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
    ) -> Option<Self> {
	let boxes_needed = (how_many + 25 - 1) / 25;

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
	).unwrap();

	Self::get_by_id(connection, connection.last_insert_rowid() as i32)
    }

    pub fn get_by_id(connection: &Connection, id: i32) -> Option<Self> {
	let mut stmt = connection.prepare(
	    "SELECT * FROM CustomerOrder WHERE id = ?1;"
	).unwrap();

	let order = stmt.query_row([id], |row| {
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
		raffle_number: row.get(8).unwrap(),
	    })
	}).unwrap().map(|o| o.unwrap()).collect()
    }

    pub fn delete(self, connection: &Connection) -> Result<usize, Error> {
	connection.execute(
	    "DELETE FROM CustomerOrder WHERE id = ?1", [self.id]
	)
    }
}

