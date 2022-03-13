// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[macro_use]
extern crate macro_rules_attribute;

#[macro_use]
extern crate rust_i18n;
i18n!("./locales");

pub mod database;
pub mod handler;
pub mod handlers;
pub mod language;

pub use handler::handle_update;
