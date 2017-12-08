extern crate day_04_high_entropy_passphrases;
extern crate utils;

use day_04_high_entropy_passphrases::high_entropy_pass;
use utils::{file2str, str2linevec};


fn main() {
    let puzzle_string = file2str("puzzle.txt");
    let lines = str2linevec(&puzzle_string);
    let mut count = 0;
    for line in lines {
        if high_entropy_pass(&line) {
            count += 1;
        }
    }
    println!("Number of valid passhprases: {}", count);
}
