// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_must_use)]

use rusqlite::{Connection, Error, params};


#[derive(Debug, Clone)]
pub struct User {
    // Telegram user ID
    id: i64,
    // Telegram user name
    name: String,
    // Bot user language
    language: String,
}

impl User {
    // Create the `users` table
    pub fn create(conn: &Connection) -> Result<(), Error> {
        let sql = "
        CREATE TABLE IF NOT EXISTS users (
                id         INTEGER PRIMARY KEY,
                name       TEXT NOT NULL,
                language   VARCHAR(5) NOT NULL DEFAULT \"en\"
        )
        ";
        
        conn.execute(sql, []);
        
        Ok(())
    }
    
    // Get a `user` by id
    pub fn get(conn: &Connection, id: i64) -> Result<Self, Error> {
        let sql = "
        SELECT * FROM users WHERE id = ?
        ";
        
        conn.query_row(sql, params![id], |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?,
                language: row.get(2)?,
            })
        })
    }
    
    // Delete a `user` by id
    pub fn delete(conn: &Connection, id: i64) -> Result<(), Error> {
        let sql = "
        DELETE FROM users WHERE id = ?
        ";
        
        conn.execute(sql, params![id]);
        
        Ok(())
    }
}
