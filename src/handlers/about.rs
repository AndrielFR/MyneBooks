// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{InputMessage, types};

use crate::utils;
use crate::dyn_async;
use crate::handler::{Data, Register};
use crate::language::I18n;


#[macro_rules_attribute(dyn_async!)]
async fn about_message<'fut>(data: &'fut Data) {
    let message = data.message.unwrap();
    let lang = data.language;
    let me = data.me;

    message
        .reply(get_about_message(lang, me))
        .await
        .expect("Failed to reply the message");
}

#[macro_rules_attribute(dyn_async!)]
async fn about_callback<'fut>(data: &'fut Data) {
    let callback = data.callback.unwrap();
    let lang = data.language;
    let me = data.me;

    // callback
    //     .answer()
    //     .edit(get_about_message(lang, me))
    //     .await
    //     .expect("Failed to answer the callback");
}

fn get_about_message(lang: I18n, me: &types::User) -> InputMessage {
    InputMessage::html(lang.get_text("texts.about", vec![("bot_name", me.first_name())]))
        .reply_markup(&utils::make_keyboard(
            vec![vec![(&lang.get_text("buttons.back", vec![]), "start$")]]
        ))
}


pub fn initialize<'a>() -> Register<'a> {
    Register::new()
        .set_name("about")
        .set_enabled(true)
        .append("message", about_message, "about$", true, Some("See about the bot"), Some(false))
        .append("callback", about_callback, "about$", false, None, None)
        .build()
}
