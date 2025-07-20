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

#[derive(Serialize)]
pub struct UserRole {
    pub id: uuid::Uuid,
    pub role: String,
}

pub trait GetInfo {
    fn get(self, data: &Vec<UserData>) -> UserData;
}

#[derive(Serialize)]
pub struct TeamResponse {
    pub side: String,
    pub users: Vec<UserRole>,
}

pub struct Team {
    pub side: String,
    pub users: Vec<UserData>,
}

pub trait SkillMedian {
    fn calc_skill_median(&self, user_data: &HashMap<Uuid, UserData>) -> f64;
}

#[derive(Serialize)]
pub struct Match {
    pub match_id: String,
    pub teams: Vec<TeamResponse>,
}

pub trait Fairness {
    fn calc_fairness(&self, data: &HashMap<Uuid, UserData>) -> i64;
}