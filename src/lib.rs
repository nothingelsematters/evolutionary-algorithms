use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod algorithm;
pub mod draw;
pub mod function;

// Run analysis result: fitness -> (number of launches with this max fitness, 1 position number).
lazy_static! {
    pub static ref MAP: Mutex<HashMap<i64, (usize, usize)>> = Mutex::new(HashMap::new());
}
