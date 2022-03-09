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
    // Open a new connection
    pub fn load_file_or_create(path: String) -> Result<Self, Error> {
        let conn = Connection::open(path).unwrap();
        
        Ok(Self {
            connection: conn,
        })
    }
    
    // Close the connection
    pub fn close(self) -> Result<(), Error> {
        Ok(self.connection.close().unwrap())
    }
    
    // Get the connection
    pub fn get_conn(&self) -> &Connection {
        &self.connection
    }
    
    // Create database tables if they do not exist
    pub fn initialize(&self) -> Result<(), Error> {
        let conn = self.get_conn();
        
        tables::User::create(&conn);
        
        Ok(())
    }
}
