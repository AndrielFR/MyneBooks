// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_must_use)]

use rusqlite::{Error, params};

use crate::database;


#[derive(Debug, Clone)]
pub struct User {
    // Telegram user ID
    pub id: i64,
    // Telegram user name
    pub name: String,
    // Bot user language
    pub language: String,
}

impl User {
    // Create the `users` table
    pub fn create() -> Result<(), Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();

        let sql = "
        CREATE TABLE IF NOT EXISTS users (
                id         INTEGER PRIMARY KEY,
                name       TEXT NOT NULL,
                language   VARCHAR(6) NOT NULL DEFAULT \"en-GB\"
        )
        ";

        conn.execute(sql, []);

        Ok(())
    }

    // Register a `user`
    pub fn register(id: i64, name: String, language: Option<&str>) -> Result<(), Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();

        let sql = "
        INSERT INTO users (id, name, language) VALUES (?, ?, ?)
        ";

        conn.execute(sql, params![id, name, language]);

        Ok(())
    }

    // Get a `user` by id
    pub fn get(id: i64) -> Result<Self, Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();

        let sql = "
        SELECT * FROM users WHERE id = ?
        ";

        conn.query_row(sql, params![id], |row| {
            Ok(Self {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                language: row.get(2).unwrap(),
            })
        })
    }

    // Delete a `user` by id
    pub fn delete(id: i64) -> Result<(), Error> {
        let dbc = database::connect().unwrap();
        let conn = dbc.get_conn();

        let sql = "
        DELETE FROM users WHERE id = ?
        ";

        conn.execute(sql, params![id]);

        Ok(())
    }
}
