extern crate day_02_corruption_checksum;
use day_02_corruption_checksum::corr_checksum;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "puzzle.txt";
    let mut file = File::open(filename).expect("file not found");
    let mut puzzle_string = String::new();
    file.read_to_string(&mut puzzle_string)
        .expect("error reading file");
    let checksum = corr_checksum(&puzzle_string);
    println!("The checksum is: {}", checksum);
}
