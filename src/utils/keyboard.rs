// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{button, reply_markup};


pub fn make_keyboard(buttons: Vec<Vec<(&str, &str)>>) -> reply_markup::Inline {
    let mut keyboard = Vec::new();

    let iter = buttons.iter();
    for line in iter {
        let mut row = Vec::new();
        for btn in line.iter() {
            row.push(button::inline(btn.0, btn.1.to_string().as_bytes()));
        }
        keyboard.push(row);
    }

    reply_markup::inline(keyboard)
}
