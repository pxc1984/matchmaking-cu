use serde::{
    Deserialize,
    Serialize,
};
use crate::services::epoch::Epoch;

#[derive(Deserialize)]
pub struct SubmitTeamsResponse {
    pub new_epoch: Epoch,
    pub is_last_epoch: bool,
}

#[derive(Serialize)]
pub struct UserRole {
    id: String,
    role: String,
}

#[derive(Serialize)]
pub struct Team {
    side: String,
    users: Vec<UserRole>,
}

#[derive(Serialize)]
pub struct Match {
    match_id: String,
    teams: Vec<Team>,
}