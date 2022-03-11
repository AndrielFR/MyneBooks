// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[macro_use]
extern crate macro_rules_attribute;

pub mod database;
pub mod handler;
pub mod handlers;

pub use handler::handle_update;
