// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{InputMessage, types};

use crate::utils;
use crate::dyn_async;
use crate::handler::{Data, HandlerOptions, Register};
use crate::language::I18n;


#[macro_rules_attribute(dyn_async!)]
async fn start_message<'fut>(data: &'fut Data) {
    let client = data.client;
    let message = data.message.unwrap();
    let lang = data.language;
    let me = data.me;

    client
        .send_message(&message.chat(), get_start_message(lang, me).reply_to(Some(message.id())))
        .await
        .expect("Failed to reply the message");
}

#[macro_rules_attribute(dyn_async!)]
async fn start_callback<'fut>(data: &'fut Data) {
    let client = data.client;
    let callback = data.callback.unwrap();
    let lang = data.language;
    let me = data.me;

    let message = callback.load_message().await.unwrap();

    client
        .edit_message(callback.chat(), message.id(), get_start_message(lang, me))
        .await
        .expect("Failed to answer the callback");
}

fn get_start_message(lang: I18n, me: &types::User) -> InputMessage {
    InputMessage::html(lang.get_text("texts.start", vec![("bot_username", me.username().unwrap())]))
        .reply_markup(&utils::make_keyboard(
            vec![vec![(&lang.get_text("buttons.about", vec![]), "about")]]
        ))
}


pub fn initialize<'a>() -> Register<'a> {
    Register::new()
        .set_name("start")
        .set_enabled(true)
        .append("message", start_message, "start$", Some(HandlerOptions { is_command: true, description: Some("Start the bot"), ..Default::default() }))
        .append("callback", start_callback, "^start$", None)
        .build()
}
