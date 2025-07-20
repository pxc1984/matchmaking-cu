use std::collections::HashMap;
use rand::Rng;
use rand::prelude::*;
use uuid::Uuid;
use crate::constants::*;
use crate::models::submit::*;
use crate::models::user::UserData;

/// Функция, которая распределяет пользователей по командам в зависимости от их:
/// - рейтинга (ММР)
/// - предпочитаемых позиций
/// TODO: - того, сколько времени они ждали
pub fn determine(all_users: &Vec<UserData>) -> Vec<Match> {
    let mut formed_matches: Vec<Match> = Vec::new();
    let mut users_by_role: HashMap<String, Vec<UserData>> = users_by_role(&all_users);

    // Сортируем каждую роль по ММР
    for players in users_by_role.values_mut() {
        players.sort_by(|a, b| b.mmr.cmp(&a.mmr));
    }

    loop {
        // Проверяем, есть ли у нас достаточно игроков для одного матча (хотяб 2е на каждую роль)
        if users_by_role.values().any(|v| v.len() < 2) || users_by_role.len() < TEAM_SIZE {
            break; // Недостаточно игроков даже на один нормальный матч
        }
        let mut candidate_pool: HashMap<String, Vec<UserData>> = HashMap::new();
        for (role, players) in &users_by_role {
            let candidates: Vec<UserData> = players.iter().take(CANDIDATES_PER_ROLE).cloned().collect();
            if candidates.len() < 2 {
                // На случай если очередь иссякнет
                return formed_matches;
            }
            candidate_pool.insert(role.clone(), candidates);
        }

        // --- Simulated Annealing чтоб найти лучший матч ---
        let (best_team1, best_team2) = find_best_teams(&candidate_pool);

        // --- Фигня для нетворкинга и тестирующей системы ---
        let match_players_ids: Vec<Uuid> = best_team1.users.values().chain(best_team2.users.values()).map(|p| p.user_id).collect();

        let team1_response = TeamResponse {
            side: "red".to_string(),
            users: best_team1.users.iter().map(|(role, user)| UserRole { id: user.user_id, role: role.clone() }).collect(),
        };
        let team2_response = TeamResponse {
            side: "blue".to_string(),
            users: best_team2.users.iter().map(|(role, user)| UserRole { id: user.user_id, role: role.clone() }).collect(),
        };

        formed_matches.push(Match {
            match_id: Uuid::new_v4().to_string(),
            teams: vec![team1_response, team2_response],
        });

        // Удаляем уже выбранных игроков из очередей
        for (role, players) in users_by_role.iter_mut() {
            players.retain(|p| !match_players_ids.contains(&p.user_id));
        }
    }

    formed_matches
}

/// Переводим список пользователей в словарь по их уникальному айди
fn users_to_data(users: Vec<UserData>) -> HashMap<Uuid, UserData> {
    let mut data = HashMap::new();
    for user in users {
        data.insert(user.user_id, user);
    }
    data
}

/// Группируем пользователей по их предпочтительной роли
fn users_by_role(users: &Vec<UserData>) -> HashMap<String, Vec<UserData>> {
    let mut users_by_role: HashMap<String, Vec<UserData>> = HashMap::new();
    for user in users.iter() {
        if let Some(primary_role) = user.roles.first() {
            users_by_role
                .entry(primary_role.clone())
                .or_default()
                .push(user.clone());
        }
    }
    users_by_role
}

/// Просто подсчет медианного значения из списка
fn get_median(list: &Vec<u32>) -> f64 {
    if list.is_empty() { return f64::NAN; }
    let mut list = list.clone();
    list.sort();
    if list.len() % 2 == 1 {
        list[list.len() / 2 + 1].clone() as f64
    } else {
        (&list[list.len() / 2] + &list[list.len() / 2 + 1]).clone() as f64
    }
}

/// Подсчитывает сумму модулей разностей ММР для каждой роли.
fn calc_skill_delta_by_role(team1: &Team, team2: &Team) -> i64 {
    let mut total_delta: i64 = 0;
    for (role, player1) in &team1.users {
        if let Some(player2) = team2.users.get(role) {
            total_delta += (player1.mmr as i64 - player2.mmr as i64).abs();
        }
    }
    total_delta
}

/// Подсчитывает честность для потенциального матча. Чем меньше, тем лучше.
/// Это наша функция стоимости для оптимизации.
fn calculate_match_fairness(team1: &Team, team2: &Team) -> f64 {
    let team1_mmrs: Vec<u32> = team1.users.values().map(|p| p.mmr).collect();
    let team2_mmrs: Vec<u32> = team2.users.values().map(|p| p.mmr).collect();

    let role_delta = calc_skill_delta_by_role(team1, team2) as f64;
    let median_delta = (get_median(&team1_mmrs) - get_median(&team2_mmrs)).abs();

    // Это генеральный рейтинг того, насколько матч честный. Чем меньше тем лучше
    role_delta + median_delta
}

/// Ядро оптимизации матчей с использованием алгоритма (Simulated Annealing)[https://en.wikipedia.org/wiki/Simulated_annealing]
fn find_best_teams(candidate_pool: &HashMap<String, Vec<UserData>>) -> (Team, Team) {
    let mut rng = rand::rng();

    let mut current_temp = INITIAL_TEMP;

    // 1. Создаем случайную расстановку
    let (mut current_team1, mut current_team2) = create_random_teams(candidate_pool, &mut rng);
    let mut current_fairness = calculate_match_fairness(&current_team1, &current_team2);

    let mut best_team1 = current_team1.clone();
    let mut best_team2 = current_team2.clone();
    let mut best_fairness = current_fairness;

    for _ in 0..ITERATIONS {
        if current_temp <= 1.0 { break; }

        // 2. Создаем "соседское" решение путем добавления маленького изменения
        let (next_team1, next_team2) = create_neighbor(&current_team1, &current_team2, candidate_pool, &mut rng);
        let next_fairness = calculate_match_fairness(&next_team1, &next_team2);

        // 3. Решаем, оставить старое или переключиться на новое решение
        if next_fairness < current_fairness || rng.random::<f64>() < ((current_fairness - next_fairness) / current_temp).exp() {
            current_team1 = next_team1;
            current_team2 = next_team2;
            current_fairness = next_fairness;
        }

        // Помним наилучшее решение из всех что находили
        if current_fairness < best_fairness {
            best_fairness = current_fairness;
            best_team1 = current_team1.clone();
            best_team2 = current_team2.clone();
        }

        current_temp *= COOLING_RATE;
    }

    (best_team1, best_team2)
}

/// Создает 2 корректные команды путем выбора случайных игроков из пула.
fn create_random_teams(pool: &HashMap<String, Vec<UserData>>, rng: &mut impl Rng) -> (Team, Team) {
    let mut team1 = Team {
        side: "red".to_string(),
        users: HashMap::new(),
    };
    let mut team2 = Team {
        side: "blue".to_string(),
        users: HashMap::new(),
    };

    for (role, candidates) in pool {
        let chosen_pair: Vec<_> = candidates.choose_multiple(rng, 2).cloned().collect();
        team1.users.insert(role.clone(), chosen_pair[0].clone());
        team2.users.insert(role.clone(), chosen_pair[1].clone());
    }
    (team1, team2)
}

/// Создает потенциальное решение путем изменения текущего.
fn create_neighbor(team1: &Team, team2: &Team, pool: &HashMap<String, Vec<UserData>>, rng: &mut impl Rng) -> (Team, Team) {
    let mut new_team1 = team1.clone();
    let mut new_team2 = team2.clone();

    // Выбираем случайную роль, чтоб изменить
    let roles: Vec<_> = pool.keys().collect();
    let role_to_swap = roles.choose(rng).unwrap();

    // 50% шанс просто поменять игроков командами
    if rng.random::<bool>() {
        let player1 = new_team1.users.get(*role_to_swap).unwrap().clone();
        let player2 = new_team2.users.get(*role_to_swap).unwrap().clone();
        new_team1.users.insert((*role_to_swap).clone(), player2);
        new_team2.users.insert((*role_to_swap).clone(), player1);
    }
    // 50% шанс поменять игрока на кого-то другого из пула
    else {
        let current_p1 = new_team1.users.get(*role_to_swap).unwrap();
        let current_p2 = new_team2.users.get(*role_to_swap).unwrap();

        // Ищем кандидата, которого еще не пытались определить на эту роль
        if let Some(new_player) = pool.get(*role_to_swap).unwrap().iter().find(|p| p.user_id != current_p1.user_id && p.user_id != current_p2.user_id) {
            // Поменять местами с игроком из красной команды
             if rng.random::<bool>() {
                new_team1.users.insert((*role_to_swap).clone(), new_player.clone());
             } else { // поменять местами с игроком из синей команды
                new_team2.users.insert((*role_to_swap).clone(), new_player.clone());
             }
        }
    }

    (new_team1, new_team2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_median() {
        assert_eq!(get_median(&vec![1, 2, 3]), 2f64);
        assert_eq!(get_median(&vec![1, 2, 3, 4]), 2.5);
        assert_eq!(get_median(&vec![3, 2, 1, 4]), 2.5);
    }
}