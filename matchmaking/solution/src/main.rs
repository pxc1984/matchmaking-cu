mod services;

use std::string::ToString;
use tracing::{debug, error, info};
use tracing_subscriber;
use reqwest;

use std::thread;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use reqwest::blocking::Client;

const SERVER_NAME: &str = "127.0.0.1:8000";

fn main() {
    tracing_subscriber::fmt().init();
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
                info!("Ping attempt #{}", attempt_number);
                match client_ref.get(&url).send() {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("Successfully pinged {}. (Status: {})", url, response.status());
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
                        error!("Error fetching {}: {}", url, e);
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


}

fn get_url(endpoint_path: &str) -> String {
    SERVER_NAME.to_string() + endpoint_path
}
