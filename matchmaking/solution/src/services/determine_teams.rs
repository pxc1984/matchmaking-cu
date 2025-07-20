use std::collections::HashMap;
use uuid::Uuid;
use crate::models::submit::*;
use crate::models::user::UserData;

/// Функция, которая распределяет пользователей по командам в зависимости от их:
/// - рейтинга (ММР)
/// - предпочитаемых позиций
/// - того, сколько времени они ждали
pub fn determine(users: Vec<UserData>) -> Vec<Match> {
    let data = users_to_data(users);

    vec![] // placeholder
}

fn users_to_data(users: Vec<UserData>) -> HashMap<Uuid, UserData> {
    let mut data = HashMap::new();
    for user in users {
        data.insert(user.user_id, user);
    }
    data
}


impl GetInfo for UserRole {
    /// get the entry from `data` where user_id is the same as `self.id`
    fn get(&self, user_data: &HashMap<Uuid, UserData>) -> UserData {
        user_data.get(&self.id).unwrap().clone()
    }
}

fn get_median(list: Vec<u32>) -> f64
{
    if list.is_empty() { return f64::NAN; }
    let mut list = list;
    list.sort();
    if list.len() % 2 == 1 {
        list[list.len() / 2 + 1].clone() as f64
    } else {
        (&list[list.len() / 2] + &list[list.len() / 2 + 1]).clone() as f64
    }
}

impl SkillMedian for TeamResponse {
    fn calc_skill_median(&self, user_data: &HashMap<Uuid, UserData>) -> f64 {
        let mut skill_levels: Vec<u32> = Vec::with_capacity(self.users.len());
        for user in self.users {
            let data = user.get(&user_data);
            skill_levels.push(data.mmr);
        }
        get_median(skill_levels.clone())
    }
}

impl SkillMedian for Team {
    fn calc_skill_median(&self, user_data: &HashMap<Uuid, UserData>) -> f64 {
        let skill_levels: Vec<u32> = self.users.iter()
            .map(|user| user.mmr)
            .collect();
        get_median(skill_levels.clone())
    }
}

fn calc_skill_delta_by_role(
    team1: &TeamResponse,
    team2: &TeamResponse,
    user_data: &HashMap<Uuid, UserData>
) -> i64 {
    let mut team1_role = HashMap::new();
    let mut role_delta = HashMap::new();
    for user in team1.users {
        let data = user.get(&user_data);
        team1_role.insert(user.role, data.mmr);
    }
    for user in team2.users {
        let data = user.get(&user_data);
        let entry = team1_role.get(&user.role).expect("found role that didn't match").clone();
        role_delta.insert(
            user.role,
            match entry > data.mmr {
                true => {entry as i64 - data.mmr as i64}
                false => {data.mmr as i64 - entry as i64}
            }
        );
    }

    role_delta
        .iter()
        .map(|key, value| {
            value
        })
        .sum()
}

fn calc_team_delta(
    team1: &TeamResponse,
    team2: &TeamResponse,
    user_data: &HashMap<Uuid, UserData>
) -> i64 {
    calc_skill_delta_by_role(&team1, &team2, &user_data).abs()
        + (team1.calc_skill_median(&user_data) - team2.calc_skill_median(&user_data)).abs() as i64
}

impl Fairness for Match {
    fn calc_fairness(&self, data: &HashMap<Uuid, UserData>) -> i64 {
        assert_eq!(self.teams.len(), 2);
        calc_team_delta(
            &self.teams[0],
            &self.teams[1],
            &data,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_median() {
        assert_eq!(get_median(vec![1, 2, 3]), 2f64);
        assert_eq!(get_median(vec![1, 2, 3, 4]), 2.5);
        assert_eq!(get_median(vec![3, 2, 1, 4]), 2.5);
    }
}