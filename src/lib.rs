#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

use wasm_bindgen::prelude::*;

#[cfg(feature = "parallel")]
#[wasm_bindgen]
pub fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.par_iter().map(|x| x * x).sum()
}

#[wasm_bindgen]
pub fn sum_of_squares_st(numbers: &[i32]) -> i32 {
    numbers.iter().map(|x| x * x).sum()
}

#[wasm_bindgen]
pub fn sum_of_squares_st_shitty(numbers: &[i32], numbers_: &[i32]) -> i32 {
    numbers.iter().map(|x| x * x).sum()
}

#[wasm_bindgen]
pub fn with_no_args() -> i32 {
    5
}
