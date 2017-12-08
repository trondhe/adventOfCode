extern crate day_01_inverse_captcha;
extern crate utils;
use day_01_inverse_captcha::inv_captcha;
use utils::{file2str, str2vec_u32};

fn main() {
    let puzzle_string = file2str("puzzle.txt");
    let puzzle_vec = str2vec_u32(&puzzle_string);
    let sum = inv_captcha(puzzle_vec);
    println!("The sum is: {}", sum);
}
