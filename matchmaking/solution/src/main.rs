mod services;
mod constants;
mod models;

use tracing::{debug, error, info, warn};
use tracing_subscriber;

use std::thread;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use reqwest::blocking::*;

use crate::constants::SERVER_NAME;
use crate::services::*;
use crate::services::get_url::*;
use crate::services::test_conn::test_conn;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // override log level from env variable
        .init();
    info!("Solution set up");

    let success = Arc::new(Mutex::new(false));

    // ожидаем, пока сервер не ответит на пинг
    test_conn(success.clone());

    if !*success.lock().unwrap() {
        error!("Failed to ping {}", get_url("/ping"));
        return;
    }

    info!("Connection to test system running on {} set up successfully", SERVER_NAME);

    for test_number in 0..20 {
        let users = get_waiting_users::get(&test_name_from_int(test_number), None);
    }
}

fn test_name_from_int(test_number: u32) -> String {
    "test_".to_string() + &test_number.to_string()
}
