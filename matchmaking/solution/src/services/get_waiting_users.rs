﻿use tracing::{debug, error, info, warn};

use std::thread;
use std::sync::{Arc, Mutex};
use reqwest::blocking::*;

use crate::models::user::UserData;
use crate::models::epoch::*;
use super::get_url::*;

pub fn get(test_name: &str, input_epoch: Option<Epoch>) -> Vec<UserData> {
    let client = Arc::new(Client::new());
    let epoch = Arc::new(input_epoch.unwrap_or_else(|| Epoch::new()));

    let users: Arc<Mutex<Vec<UserData>>> = Arc::new(Mutex::new(vec![]));

    thread::scope(|s| {
        s.spawn(|| {
            let client_ref = Arc::clone(&client);
            let epoch_ref = Arc::clone(&epoch);

            // let url = get_url("/matchmaking/users");
            let url = get_url_params(
                "/matchmaking/users",
                vec![
                    ("test_name", test_name),
                    ("epoch", &format!("{}", epoch_ref))
                ]
            );
            debug!("Attempting get_waiting_users with url {}", url);
            match client_ref.get(&url).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<Vec<UserData>>() {
                            Ok(parsed_users) => {
                                debug!("Got {} users from {}", parsed_users.len(), url);

                                let users_arc = Arc::clone(&users);
                                let mut users_ref = users_arc
                                    .lock()
                                    .expect("failed to unwrap users_ref lock");

                                users_ref.extend(parsed_users);
                            }
                            Err(e) => {
                                error!("Failed to parse JSON response from {}: {}", url, e);
                            }
                        }
                    } else {
                        let status = response.status();
                        let text = response.text().unwrap_or_else(|_| "N/A".to_string());
                        error!("Server returned error for {}: Status {} - Body: {}", url, status, text);
                    }
                }
                Err(e) => {
                    error!("Failed to get waiting users by {}: {}", url, e);
                }
            }
        });
    });

    users
        .lock()
        .expect("failed to unwrap ret lock")
        .clone()
}