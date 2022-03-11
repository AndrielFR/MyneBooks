// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_must_use)]

use rusqlite::{Error, params};

use crate::database;


#[derive(Debug, Clone)]
pub struct Group {
    // Telegram group ID
    id: i64,
    // Telegram group title
    title: String,
    // Bot group language
    language: String,
}

impl Group {
    // Create the `groups` table
    pub fn create() -> Result<(), Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();
        
        let sql = "
        CREATE TABLE IF NOT EXISTS groups (
                id         INTEGER PRIMARY KEY,
                title      TEXT NOT NULL,
                language   VARCHAR(5) NOT NULL DEFAULT \"en\"
        )
        ";
        
        conn.execute(sql, []);
        
        Ok(())
    }
    
    // Register a `group`
    pub fn register(id: i64, title: &str) -> Result<(), Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();
        
        let sql = "
        INSERT INTO groups (id, title) VALUES (?, ?)
        ";
        
        conn.execute(sql, params![id, title]);
        
        Ok(())
    }
    
    // Get a `group` by id
    pub fn get(id: i64) -> Result<Self, Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();
        
        let sql = "
        SELECT * FROM groups WHERE id = ?
        ";
    
        conn.query_row(sql, params![id], |row| {
            Ok(Self {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                language: row.get(2).unwrap(),
            })
        })
    }
    
    // Delete a `group` by id
    pub fn delete(id: i64) -> Result<(), Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();
        
        let sql = "
        DELETE FROM groups WHERE id = ?
        ";
        
        conn.execute(sql, params![id]);
        
        Ok(())
    }
}
