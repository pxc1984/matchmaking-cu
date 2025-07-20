use std::collections::HashMap;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::models::user::UserData;
use crate::services::epoch::Epoch;

#[derive(Deserialize)]
pub struct SubmitTeamsResponse {
    pub new_epoch: Epoch,
    pub is_last_epoch: bool,
}

#[derive(Serialize, Clone)]
pub struct UserRole {
    pub id: Uuid,
    pub role: String,
}

#[derive(Serialize)]
pub struct TeamResponse {
    pub side: String,
    pub users: Vec<UserRole>,
}

#[derive(Clone)]
pub struct Team {
    pub side: String,
    pub users: HashMap<String, UserData>,
}

#[derive(Serialize)]
pub struct Match {
    pub match_id: String,
    pub teams: Vec<TeamResponse>,
}