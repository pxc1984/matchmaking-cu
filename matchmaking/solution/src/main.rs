mod services;
mod constants;

use tracing::{debug, error, info, warn};
use tracing_subscriber;

use std::thread;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use reqwest::blocking::*;
use crate::constants::SERVER_NAME;
use crate::services::*;
use crate::services::get_url::*;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // override log level from env variable
        .init();
    info!("Solution set up");

    let client = Arc::new(Client::new());
    let success = Arc::new(Mutex::new(false));

    // ожидаем, пока сервер не ответит на пинг
    thread::scope(|s| {
        let client_ref = Arc::clone(&client);
        let success = Arc::clone(&success);

        s.spawn(move || {
            debug!("Attempting to ping test system");
            let url = get_url("/ping");
            for attempt_number in 1..=10 {
                debug!("Ping attempt #{}", attempt_number);
                match client_ref.get(&url).send() {
                    Ok(response) => {
                        if response.status().is_success() {
                            debug!("Successfully pinged {}. (Status: {})", url, response.status());
                            let mut success_lock = success
                                .lock()
                                .expect("failed to unwrap success lock");
                            *success_lock = true;
                            break;
                        } else {
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                    Err(e) => {
                        warn!("Error fetching {}: {}", url, e);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        });
    });

    if !*success.lock().unwrap() {
        error!("Failed to ping {}", get_url("/ping"));
        return;
    }

    info!("Connection to test system running on {} set up successfully", SERVER_NAME);


}
