// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::dyn_async;
use crate::handler::{Data, Register};


#[macro_rules_attribute(dyn_async!)]
async fn start<'fut>(data: &'fut Data) {
    let client = data.client;
    let message = data.message;
    let chat = message.chat();
    
    client.send_message(&chat, message.text()).await
        .expect("Failed to reply the message");
}


pub fn initialize<'a>() -> Register<'a> {
    Register::new()
        .set_name("start")
        .set_enabled(true)
        .append("message", start, "start$", true, Some("Start the bot"), Some(false))
        .build()
}
