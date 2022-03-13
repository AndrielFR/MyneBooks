// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::dyn_async;
use crate::handler::{Data, Register};


#[macro_rules_attribute(dyn_async!)]
async fn start<'fut>(data: &'fut Data) {
    let client = data.client;
    let message = data.message.unwrap();
    let lang = data.language;
    let me = data.me;

    let chat = message.chat();

    client.send_message(&chat, lang
        .get_text("texts.start", vec![("bot_username", me.username().unwrap())])).await
        .expect("Failed to reply the message");
}


pub fn initialize<'a>() -> Register<'a> {
    Register::new()
        .set_name("start")
        .set_enabled(true)
        .append("message", start, "start$", true, Some("Start the bot"), Some(false))
        .build()
}
