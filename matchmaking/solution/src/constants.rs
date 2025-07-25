﻿use std::env;
use std::sync::LazyLock;

/// Где находится тестирующая система?
// pub const SERVER_NAME: &str = "127.0.0.1:8000";
pub static SERVER_NAME: LazyLock<String> = LazyLock::new(|| match env::var("SERVER_NAME") {
    Ok(val) => { val.clone() }
    Err(_) => { "127.0.0.1:8000".to_string() }
});
// --- Банальные константы чтоб не хардкодить ---
pub const TEAM_SIZE: usize = 5;
pub const PLAYERS_PER_MATCH: usize = TEAM_SIZE * 2;
pub const THREAD_COUNT: usize = 20;

// --- Пул потенциальных решений ---
/// Берет N лучших игроков для каждой роли для матча
/// Это делает поиск возможным. Значения от 4-6 пойдут.
pub const CANDIDATES_PER_ROLE: usize = 4;

// --- Параметры (SA)[https://en.wikipedia.org/wiki/Simulated_annealing] ---
// Эти значения могут влиять на эффективность алгоритма, но, к сожалению, универсальной
// расстановки их нет(
pub const INITIAL_TEMP: f64 = 1000.0;
pub const COOLING_RATE: f64 = 0.995;
pub const ITERATIONS: usize = 2000;
