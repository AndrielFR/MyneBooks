// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use grammers_client::{Client, Update, types};
use regex::Regex;
use log::info;

use crate::handlers;
// use crate::database;
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
    pub message: &'a types::Message,
    // pub callback: &'a types::Callback,
    // pub inline: &'a types::Inline,
}

#[derive(Clone)]
pub struct Handler<'a> {
    update_type: &'a str,
    function: AsyncFunction,
    pattern: &'a str,
    is_command: bool,
    description: Option<&'a str>,
    hide: Option<bool>,
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
    
    pub fn append(mut self, update_type: &'a str, function: AsyncFunction, pattern: &'a str, is_command: bool, description: Option<&'a str>, hide: Option<bool>) -> Self {
        let handler = Handler {
            update_type: update_type,
            function: function,
            pattern: pattern,
            is_command: is_command,
            description: description,
            hide: hide,
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

pub async fn handle_update<'a>(client: Client, update: Update, handler_list: Vec<Handler<'a>>, prefixes: Vec<String>) -> Result<(), Box<dyn Error>> {
    // let dbc = database::connect().unwrap();
    // let conn = dbc.get_conn();
    
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
                }
                types::Chat::Group(group) => {
                    match tables::Group::get(group.id()) {
                        Ok(_) => {}
                        Err(_) => {
                            tables::Group::register(group.id(), group.title());
                        }
                    }
                }
                types::Chat::Channel(channel) => {}
            }
            
            let message_handlers = handler_list.iter()
                .filter(|handler| handler.update_type == "message");
            for handler in message_handlers {
                let function = handler.function;
                let pattern = handler.pattern;
                let is_command = handler.is_command;
                
                let data = Data {
                    client: &client,
                    message: message,
                };
                
                if is_command {
                    let pattern = format!("[{}]{}", prefixes.join(""), pattern).as_str();
                }
                
                let re = Regex::new(pattern).unwrap();
                if re.is_match(message.text()) {
                    function(&data).await;
                }
            }
        }
        _ => {}
    }
    
    Ok(())
}
