// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(unused_must_use)]

use rusqlite::{Connection, Error};

use crate::database::tables;


// A client to do the SQL operations.
pub struct Client {
    connection: Connection,
}

impl Client {
    // Close the connection
    pub fn close(self) -> Result<(), Error> {
        self.connection.close().unwrap();
        
        Ok(())
    }
    
    // Get the connection
    pub fn get_conn(&self) -> &Connection {
        &self.connection
    }
    
    // Create database tables if they do not exist
    pub fn initialize(&self) -> Result<(), Error> {
        tables::User::create().expect("Failed to create the `users` table");
        tables::Group::create().expect("Failed to create the `groups` table");
        
        Ok(())
    }
}

// Open a new connection
pub fn connect() -> Result<Client, Error> {
    let path = "./src/database/sqlite.db3".to_string();
    let conn = Connection::open(&path).unwrap();
    
    Ok(Client {
        connection: conn,
    })
}
