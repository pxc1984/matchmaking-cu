use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use tracing::{debug, warn};
use crate::services::get_url::get_url;

/// Ждет когда сервер ответит на пинг.
/// В зависимости от результата меняет `success`
pub fn test_conn(success: Arc<Mutex<bool>>) {
    let client = Arc::new(Client::new());

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
}