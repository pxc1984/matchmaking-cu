use std::collections::HashMap;
use uuid::Uuid;
use crate::constants::*;
use crate::models::submit::*;
use crate::models::user::UserData;

/// Функция, которая распределяет пользователей по командам в зависимости от их:
/// - рейтинга (ММР)
/// - предпочитаемых позиций
/// - того, сколько времени они ждали
pub fn determine(_users: &Vec<UserData>) -> Vec<Match> {
    let _users_data = users_to_data(_users.clone());

    let mut available_users = _users.clone();

    // Сортируем пользователей по ММР по убыванию
    available_users.sort_by(|a, b| b.mmr.cmp(&a.mmr));

    let mut formed_matches: Vec<Match> = Vec::new();

    while available_users.len() >= PLAYERS_PER_MATCH {
        let mut team1_players: Vec<UserRole> = Vec::with_capacity(TEAM_SIZE);
        let mut team2_players: Vec<UserRole> = Vec::with_capacity(TEAM_SIZE);

        for i in 0..PLAYERS_PER_MATCH {
            let user = available_users.remove(0);

            let user_role = UserRole {
                id: user.user_id,
                role: user.roles.get(0).cloned().unwrap(),
            };

            if i % 4 == 0 || i % 4 == 3 {
                team1_players.push(user_role);
            } else {
                team2_players.push(user_role);
            }
        }

        // Условные обозначения: первая команда - красная, вторая команда - синяя
        let team1_response = TeamResponse {
            side: "red".to_string(),
            users: team1_players,
        };
        let team2_response = TeamResponse {
            side: "blue".to_string(),
            users: team2_players,
        };

        let new_match = Match {
            match_id: Uuid::new_v4().to_string(),
            teams: vec![team1_response, team2_response],
        };

        // Реализован чистый жадный алгоритм, в нем мы все принимаем.
        // Если будешь делать более продвинутый алгоритм, то на этом шаге надо подсчитывать
        // честность и потенциально отклонять матч или пытаться пересоставить его, если он слишком имба.
        formed_matches.push(new_match);
    }

    formed_matches
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
    fn calc_skill_median(&self, _data: &HashMap<Uuid, UserData>) -> f64 {
        let mut skill_levels: Vec<u32> = Vec::with_capacity(self.users.len());
        for user in self.users {
            let data = user.get(&_data);
            skill_levels.push(data.mmr);
        }
        get_median(skill_levels.clone())
    }
}

impl SkillMedian for Team {
    fn calc_skill_median(&self, _data: &HashMap<Uuid, UserData>) -> f64 {
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