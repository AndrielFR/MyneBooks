// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{InputMessage, types};

use crate::utils;
use crate::dyn_async;
use crate::handler::{Data, HandlerOptions, Register};
use crate::language::I18n;


#[macro_rules_attribute(dyn_async!)]
async fn about_message<'fut>(data: &'fut Data) {
    let client = data.client;
    let message = data.message.unwrap();
    let lang = data.language;
    let me = data.me;

    client
        .send_message(&message.chat(), get_about_message(lang, me).reply_to(Some(message.id())))
        .await
        .expect("Failed to reply the message");
}

#[macro_rules_attribute(dyn_async!)]
async fn about_callback<'fut>(data: &'fut Data) {
    let client = data.client;
    let callback = data.callback.unwrap();
    let lang = data.language;
    let me = data.me;

    let message = callback.load_message().await.unwrap();

    client
        .edit_message(callback.chat(), message.id(), get_about_message(lang, me))
        .await
        .expect("Failed to answer the callback");
}

fn get_about_message(lang: I18n, me: &types::User) -> InputMessage {
    InputMessage::html(lang.get_text("texts.about", vec![("bot_name", me.first_name())]))
        .reply_markup(&utils::make_keyboard(
            vec![vec![(&lang.get_text("buttons.back", vec![]), "start")]]
        ))
}


pub fn initialize<'a>() -> Register<'a> {
    Register::new()
        .set_name("about")
        .set_enabled(true)
        .append("message", about_message, "about$", Some(HandlerOptions { is_command: true, description: Some("See about the bot"), ..Default::default() }))
        .append("callback", about_callback, "^about$", None)
        .build()
}
