use tracing::{debug, error, info, warn};

use std::thread;
use std::sync::{Arc, Mutex};
use reqwest::blocking::*;

use crate::models::submit::{Match, SubmitTeamsResponse};
use super::epoch::*;
use super::get_url::*;


pub fn submit(matches: Vec<Match>, test_name: &str, epoch: Epoch) -> (Epoch, bool) {
    let client = Arc::new(Client::new());
    let test_name_arc = Arc::new(test_name);
    let epoch_arc = Arc::new(epoch);

    let ret: Arc<Mutex<(Epoch, bool)>> = Arc::new(Mutex::new((Epoch::new(), false)));

    thread::scope(|s| {
        s.spawn(|| {
            let client_ref = Arc::clone(&client);
            let test_name_ref = Arc::clone(&test_name_arc);
            let epoch_ref = Arc::clone(&epoch_arc);

            let ret_ref = Arc::clone(&ret);

            debug!("Attempting to submit commands");
            let url = get_url_params("/matchmaking/match", vec![
                ("test_name", &test_name_ref),
                ("epoch", &format!("{}", epoch_ref)),
            ]);
            match client_ref.post(&url).json(&matches).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<SubmitTeamsResponse>() {
                            Ok(response) => {
                                debug!("Got new epoch: {} and is_last_epoch is {}",
                                    response.new_epoch,
                                    response.is_last_epoch
                                );

                                let mut ret_lock = ret_ref
                                    .lock()
                                    .expect("failed to aquire lock on ret_ref");
                                *ret_lock = (response.new_epoch, response.is_last_epoch);
                            }
                            Err(e) => {
                                error!("Failed to parse JSON response from {}: {}", url, e);
                            }
                        }
                    } else {
                        let status = response.status();
                        let text = response.text().unwrap_or_else(|_| "N/A".to_string());
                        error!("Server returned error for {url}: Status {status} - Body: {text}")
                    }
                }
                Err(e) => {
                    error!("Error submitting teams to {}: {}", url, e);
                }
            }
        });
    });

    ret.lock().unwrap().clone()
}