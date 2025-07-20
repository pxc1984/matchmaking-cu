use crate::models::submit::Match;
use crate::models::user::User;

/// Функция, которая распределяет пользователей по командам в зависимости от их:
/// - рейтинга (ММР)
/// - предпочитаемых позиций
/// - того, сколько времени они ждали
pub fn determine(users: Vec<User>) -> Vec<Match> {
    vec![] // placeholder
}