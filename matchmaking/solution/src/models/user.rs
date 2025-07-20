use serde::{Deserialize, Serialize};
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    mmr: u64,
    roles: Vec<String>,
    user_id: uuid::Uuid,
    #[serde(rename = "waitingTime")]
    waiting_time: u32,
}