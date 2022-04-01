use std::sync::MutexGuard;

use libadwaita as adw;
use libadwaita::prelude::{ApplicationExt, ApplicationExtManual};
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Sender;

use crate::data::config::Settings;
use crate::events::{DataEvent, EventHandler};
use crate::events::handler::Handler;
use crate::services::microsoft::token::MicrosoftService;
use crate::services::ToDoService;

pub struct App {}

impl App {
    pub async fn login() {
        if MicrosoftService::is_token_present() {
            match MicrosoftService::current_token_data() {
                None => println!("Couldn't find current token data"),
                Some(config) => {
                    match MicrosoftService::refresh_token(config.refresh_token.as_str()).await {
                        Ok(token) => match MicrosoftService::update_token_data(&token) {
                            Ok(_) => println!("Token configuration updated."),
                            Err(err) => println!("{err}"),
                        },
                        Err(err) => println!("{err}"),
                    }
                }
            };
        } else {
            match MicrosoftService::authenticate().await {
                Ok(_) => {}
                Err(err) => println!("{err}"),
            }
        }
    }
    pub async fn uri(code: String, data_tx: &MutexGuard<'_, Sender<DataEvent>>) {
        match MicrosoftService::token(code).await {
            Ok(token_data) => match MicrosoftService::update_token_data(&token_data) {
                Ok(_) => {
                    match MicrosoftService::get_lists().await {
                        Ok(lists) => {
                            data_tx
                                .send(DataEvent::Login)
                                .await
                                .expect("Failed to send Login event.");
                            data_tx
                                .send(DataEvent::UpdateLists(lists))
                                .await
                                .expect("Failed to send Login event.");
                        }
                        Err(err) => println!("{err}"),
                    }
                    println!("Updated token data.");
                }
                Err(err) => println!("{err}"),
            },
            Err(err) => println!("{err}"),
        }
    }
    pub fn connect_events(application: &adw::Application) -> anyhow::Result<()> {
        let event_handler = EventHandler::new(channel(1), channel(1));
        let ui_tx = event_handler.clone();
        Handler::handle_events(event_handler.clone());
        Settings::config()?;
        application
            .connect_open(move |_, files, _| Handler::handle_uri(files, event_handler.clone()));
        application.connect_activate(move |app| Handler::build_ui(app, ui_tx.clone()));
        Ok(())
    }
}
