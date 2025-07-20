use std::ops::Deref;
use tracing::{debug, error, info, warn};
use tracing_subscriber;

use std::thread;
use std::sync::{ Arc, Mutex };
use std::time::{
    Duration,
};
use reqwest::blocking::*;

use super::epoch::*;
use super::get_url::*;
use super::*;

pub fn get_waiting_users(test_name: &str, input_epoch: Option<Epoch>) {
    let client = Arc::new(Client::new());
    let epoch = Arc::new(input_epoch.unwrap_or_else(|| Epoch::new()));

    thread::scope(|s| {
        let client_ref = Arc::clone(&client);
        let epoch_ref = Arc::clone(&epoch);

        s.spawn(move || {
            // let url = get_url("/matchmaking/users");
            let url = get_url_params(
                "/matchmaking/users",
                vec![
                    ("test_name", "test_0"),
                    ("epoch", &epoch_ref.repr())
                ]
            );
            debug!("Attempting get_waiting_users with url {}", url);
            match client_ref.get(&url).send() {
                Ok(response) => {
                    // TODO: this
                }
                Err(e) => {
                    error!("Failed to get waiting users by {}: {}", url, e);
                }
            }
        });
    });
}