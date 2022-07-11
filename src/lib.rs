use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod algorithm;
pub mod draw;
pub mod function;

// Run analysis result: fitness -> (number of launches with this max fitness, 1 position number).
lazy_static! {
    pub static ref COUNT: Mutex<Vec<i64>> = Mutex::new(vec![0; 512]);
    pub static ref DRIFT: Mutex<Vec<i64>> = Mutex::new(vec![0; 512]);
}
