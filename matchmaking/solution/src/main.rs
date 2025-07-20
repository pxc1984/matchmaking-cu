mod services;
mod constants;
mod models;

use tracing::{debug, error, info, warn};
use tracing_subscriber;

use std::thread;
use std::sync::{ Arc, Mutex };
use std::collections::VecDeque;

use crate::constants::{SERVER_NAME, THREAD_COUNT};
use crate::services::*;
use crate::services::epoch::Epoch;
use crate::services::get_url::*;
use crate::services::test_conn::test_conn;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO) // override log level from env variable
        .init();
    info!("Solution set up");

    let success = Arc::new(Mutex::new(false));

    // ожидаем, пока сервер не ответит на пинг
    test_conn(success.clone());

    if !*success.lock().unwrap() {
        error!("Failed to ping {}", get_url("/ping"));
        return;
    }

    info!("Connection to test system running on {} set up successfully", SERVER_NAME.to_string());

    let test_number_deque = Arc::new(Mutex::new(VecDeque::from_iter(0..20)));

    thread::scope(|s| {
        for _ in 0..THREAD_COUNT {
            let test_number_deque_ref = Arc::clone(&test_number_deque);
            s.spawn(move || {
                loop {
                    let mut queue = test_number_deque_ref.lock().unwrap();
                    let test_number_option = queue.pop_front();
                    drop(queue);
                    match test_number_option {
                        Some(test_number) => {
                            let mut running = true;
                            let mut epoch = Epoch::new();
                            while running {
                                let test_name = test_name_from_int(test_number);
                                info!("Running test #{} with epoch {}", test_number, epoch);
                                let users = get_waiting_users::get(
                                    &test_name,
                                    Some(epoch.clone())
                                );

                                let teams = determine_teams::determine(&users);
                                let (new_epoch, is_last_epoch) = post_teams::submit(teams, &test_name, epoch.clone());
                                epoch = new_epoch;
                                running = !is_last_epoch;
                            }
                        }
                        None => { break; }
                    }
                }
            });
        }
    });

    info!("All tests ran successfully. Stopping...");
}

fn test_name_from_int(test_number: u32) -> String {
    "test_".to_string() + &test_number.to_string()
}
