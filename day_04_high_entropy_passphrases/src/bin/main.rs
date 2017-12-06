extern crate day_04_high_entropy_passphrases;
extern crate utils;

use day_04_high_entropy_passphrases::high_entropy_pass;
use utils::str2linevec;

use std::fs::File;
use std::io::prelude::*;


fn main() {
    let filename = "puzzle.txt";
    let mut file = File::open(filename).expect("file not found");
    let mut puzzle_string = String::new();
    file.read_to_string(&mut puzzle_string)
        .expect("error reading file");
    let lines = str2linevec(&puzzle_string);
    let mut count = 0;
    for line in lines {
        if high_entropy_pass(&line) {
            count += 1;
        }
    }
    println!("Number of valid passhprases: {}", count);
}
