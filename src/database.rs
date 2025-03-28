use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use crate::order::Order;

/// Gets the path to the database. It creates the path to it if it doesn't exist.
fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().expect("Failed to get local data directory");
    path.push("OrderManagementToolAS91896");
    fs::create_dir_all(&path).expect("Failed to create database directory");
    path.push("orders.db");
    path
}

pub fn init_db_tables(connection: &Connection) {
    Order::init_table(&connection).unwrap();
}

/// Initializes the database and adds all the required tables.
pub fn init_db() -> Connection {
    let db_path = get_db_path();
    let connection = Connection::open(db_path).expect("Failed to open SQLite database");

    init_db_tables(&connection);
    
    connection
}
