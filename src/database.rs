use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use crate::order::Order;

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().expect("Failed to get local data directory");
    path.push("OrderManagementToolAS91896");
    fs::create_dir_all(&path).expect("Failed to create database directory");
    path.push("orders.db");
    path
}

pub fn init_db() -> Connection {
    let db_path = get_db_path();
    let connection = Connection::open(db_path).expect("Failed to open SQLite database");

    Order::init_table(&connection);

    connection
}
