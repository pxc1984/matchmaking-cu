use serde::{Deserialize, Serialize};
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub mmr: u32,
    pub roles: Vec<String>,
    pub user_id: uuid::Uuid,
    #[serde(rename = "waitingTime")]
    pub waiting_time: u32,
}