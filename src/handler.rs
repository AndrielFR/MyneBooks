// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::error::Error;

use grammers_client::{Client, Update};


pub async fn handle_update(client: Client, update: Update) -> Result<(), Box<dyn Error>> {
    match update {
        Update::NewMessage(message) if !message.outgoing() => {
            let chat = message.chat();
            client.send_message(&chat, message.text()).await?;
        }
        _ => {}
    }
    
    Ok(())
}