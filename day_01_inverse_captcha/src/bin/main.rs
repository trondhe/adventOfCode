extern crate day_01_inverse_captcha;
extern crate utils;
use day_01_inverse_captcha::inv_captcha;
use utils::str2vec_u32;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "puzzle.txt";
    let mut file = File::open(filename).expect("file not found");
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("error reading file");

    let puzzle_vec = str2vec_u32(&string);
    let sum = inv_captcha(puzzle_vec);
    println!("The sum is: {}", sum);
}
