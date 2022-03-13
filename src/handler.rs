// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_must_use)]

use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use grammers_client::{Client, Update, types};
use regex::Regex;
use log::info;

use crate::handlers;
use crate::language;
use crate::database::tables;


pub type AsyncFunction = for<'a> fn (&'a Data) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

#[macro_export]
macro_rules! dyn_async {(
    $(#[$attr:meta])*
    $pub:vis
    async fn $fname:ident<$lt:lifetime> ($($args:tt)*) $(-> $Ret:ty)? {
        $($body:tt)*
    }
) => (
    $(#[$attr])*
    #[allow(unused_parents)]
    $pub
    fn $fname<$lt> ($($args)*) -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ($($Ret)?)> + ::std::marker::Send + $lt >> {
        ::std::boxed::Box::pin(async move {
            $($body)*
        })
    }
)}

pub struct Data<'a> {
    pub client: &'a Client,
    pub message: Option<&'a types::Message>,
    pub callback: Option<&'a types::CallbackQuery>,
    // pub inline: Option<&'a types::InlineQuery>,
    pub request: &'a str,
    pub language: language::I18n<'a>,
    pub me: &'a types::User,
}

#[derive(Clone)]
pub struct Handler<'a> {
    update_type: &'a str,
    function: AsyncFunction,
    pattern: &'a str,
    options: Option<HandlerOptions<'a>>,
}

#[derive(Clone, Copy)]
pub struct HandlerOptions<'a> {
    pub is_command: bool,
    pub description: Option<&'a str>,
    pub hide: bool,
}

impl<'a> Default for HandlerOptions<'a> {
    fn default() -> Self {
        Self {
            is_command: false,
            description: None,
            hide: true,
        }
    }
}

pub struct Register<'a> {
    name: &'a str,
    enabled: bool,
    handler_list: Vec<Handler<'a>>,
}

impl<'a> Register<'a> {
    pub fn new() -> Self {
        Self {
            name: "undefined",
            enabled: false,
            handler_list: vec![],
        }
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn append(mut self, update_type: &'a str, function: AsyncFunction, pattern: &'a str, options: Option<HandlerOptions<'a>>) -> Self {
        let handler = Handler {
            update_type: update_type,
            function: function,
            pattern: pattern,
            options: options,
        };

        self.handler_list.push(handler);

        self
    }

    pub fn remove(mut self, index: usize) -> Self {
        self.handler_list.remove(index);

        self
    }

    pub fn set_name(mut self, name: &'a str) -> Self {
        self.name = name;

        self
    }

    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;

        self
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_name(&self) -> &'a str {
        &self.name
    }

    pub fn get_handler_list(self) -> Vec<Handler<'a>> {
        self.handler_list
    }
}

pub fn initialize<'a>(handler_list: &mut Vec<Handler<'a>>) -> Result<(), Box<dyn Error>> {

    for plugin_register in vec![
        handlers::start::initialize(), // start.rs
        handlers::about::initialize(),  // about.rs
    ] {
        if plugin_register.is_enabled() {
            info!("Loading plugin '{}'", plugin_register.get_name());

            for plugin_handler in plugin_register.get_handler_list() {
                handler_list.push(plugin_handler);
            }
        } else {
            info!("Plugin '{}' is disabled, passing...", plugin_register.get_name());
        }
    }

    Ok(())
}

pub async fn handle_update<'a>(mut client: Client, update: Update, handler_list: Vec<Handler<'a>>, prefixes: Vec<String>, me: types::User) -> Result<(), Box<dyn Error>> {
    let mut lang = language::I18n::default();

    match update {
        Update::NewMessage(ref message) if !message.outgoing() => {

            let chat = message.chat();
            match chat {
                types::Chat::User(user) => {
                    match tables::User::get(user.id()) {
                        Ok(_) => {}
                        Err(_) => {
                            tables::User::register(user.id(), user.full_name(), user.lang_code());
                        }
                    }

                    lang = language::from_user(user.id());
                }
                types::Chat::Group(group) => {
                    match tables::Group::get(group.id()) {
                        Ok(_) => {}
                        Err(_) => {
                            tables::Group::register(group.id(), group.title());
                        }
                    }

                    lang = language::from_group(group.id());
                }
                types::Chat::Channel(_) => {}
            }

            let message_handlers = handler_list.iter()
                .filter(|handler| handler.update_type == "message");
            for handler in message_handlers {
                let options = handler.options.unwrap_or_default();
                let function = handler.function;
                let mut pattern = String::from(handler.pattern);

                if options.is_command {
                    let mut has_final_line = false;

                    if pattern.ends_with("$") {
                        pattern.pop();
                        has_final_line = true;
                    }

                    let pattern_clone = pattern.clone();
                    let pattern_splitted: Vec<&str> = pattern_clone
                        .split_whitespace()
                        .collect();
                    if pattern_splitted.len() > 1 {
                        pattern.clear();
                        pattern.push_str(&pattern_splitted[..1]
                            .join(" "));
                    }

                    pattern.push_str(format!("(?:@{})?", me.username().unwrap()).as_str());

                    pattern.insert_str(0, format!("^[{}]", prefixes.join("")).as_str());

                    let pattern_parts = &pattern_splitted[1..];
                    for part in pattern_parts {
                        pattern.push_str(format!(" {}", part).as_str());
                    }

                    if has_final_line {
                        pattern.push_str("$");
                    }
                }

                let pattern = pattern.as_str();
                let request = message.text();

                let re = Regex::new(pattern).unwrap();
                if re.is_match(request) {
                    let data = Data {
                        client: &mut client,
                        message: Some(message),
                        callback: None,
                        request: request,
                        language: lang,
                        me: &me,
                    };
                    function(&data).await;
                }
            }
        }
        Update::CallbackQuery(ref callback) => {
            let chat = callback.chat();
            match chat {
                types::Chat::User(user) => {
                    match tables::User::get(user.id()) {
                        Ok(_) => {}
                        Err(_) => {
                            tables::User::register(user.id(), user.full_name(), user.lang_code());
                        }
                    }

                    lang = language::from_user(user.id());
                }
                types::Chat::Group(group) => {
                    match tables::Group::get(group.id()) {
                        Ok(_) => {}
                        Err(_) => {
                            tables::Group::register(group.id(), group.title());
                        }
                    }

                    lang = language::from_group(group.id());
                }
                types::Chat::Channel(_) => {}
            }

            let callback_handlers = handler_list.iter()
                .filter(|handler| handler.update_type == "callback");
            for handler in callback_handlers {
                let function = handler.function;
                let pattern = handler.pattern;

                let data = callback.data();
                let request = std::str::from_utf8(data).unwrap();

                let re = Regex::new(pattern).unwrap();
                if re.is_match(request) {
                    let data = Data {
                        client: &mut client,
                        message: None,
                        callback: Some(callback),
                        request: request,
                        language: lang,
                        me: &me,
                    };
                    function(&data).await;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
