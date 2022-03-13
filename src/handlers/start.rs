// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{InputMessage};

use crate::utils;
use crate::dyn_async;
use crate::handler::{Data, Register};


#[macro_rules_attribute(dyn_async!)]
async fn start_message<'fut>(data: &'fut Data) {
    let message = data.message.unwrap();
    let lang = data.language;
    let me = data.me;

    message
        .reply(InputMessage::html(
            lang.get_text("texts.start", vec![("bot_username", me.username().unwrap())]))
            .reply_markup(&utils::make_keyboard(
                 vec![vec![(&lang.get_text("buttons.about", vec![]), "about$")]]
            ))
        )
        .await
        .expect("Failed to reply the message");
}


pub fn initialize<'a>() -> Register<'a> {
    Register::new()
        .set_name("start")
        .set_enabled(true)
        .append("message", start_message, "start$", true, Some("Start the bot"), Some(false))
        .build()
}
