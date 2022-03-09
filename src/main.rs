// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(unused_must_use)]

use std::error::Error;
use std::fs::File;
use std::io::Read;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use grammers_client::{Client, Config as GConfig, InitParams};
use grammers_session::Session;
use tokio::{runtime, task};
use serde_derive::Deserialize;

use bookete::database;
use bookete::handle_update;


#[derive(Debug, Deserialize)]
struct TConfig {
    grammers: Grammers,
}

#[derive(Debug, Deserialize)]
struct Grammers {
    api_id: i32,
    api_hash: String,
    bot_token: String,
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let stdout = ConsoleAppender::builder().build();
    
    let config = Config::builder()
        .appender(Appender::builder()
        .build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    
    let _handle = log4rs::init_config(config).unwrap();
    
    // Connect the database
    let database_client = database::Client::load_file_or_create("./src/database/sqlite.db3".to_string())
        .unwrap();
    database_client.initialize();
    
    // Get the configuration
    let mut toml_str = String::new();
    File::open("config.toml")
        .and_then(|mut f| f.read_to_string(&mut toml_str))
        .unwrap();
    let decoded: TConfig = toml::from_str(&toml_str)?;
    let api_id = decoded.grammers.api_id;
    let api_hash = decoded.grammers.api_hash;
    let bot_token = decoded.grammers.bot_token;
    
    // Starts the bot
    let mut client = Client::connect(GConfig {
        session: Session::load_file_or_create("bookete.session")?,
        api_id: api_id,
        api_hash: api_hash.clone(),
        
        params: InitParams {
            flood_sleep_threshold: Some(120),
            ..Default::default()
        },
    })
    .await?;
    
    // Log in if haven't already
    if !client.is_authorized().await? {
        client.bot_sign_in(&bot_token, api_id, &api_hash).await?;
        client.session().save_to_file("bookete.session")?;
    }
    
    // Handle the updates
    while let Some(update) = client.next_update().await? {
        let handle = client.clone();
        task::spawn(async move {
            match handle_update(handle, update).await {
                Ok(_) => {}
                Err(e) => eprintln!("Error handling updates!: {}", e),
            }
        });
    }
    
    // Save the session and exit
    client.session().save_to_file("bookete.session")?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}