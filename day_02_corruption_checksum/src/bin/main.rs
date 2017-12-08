extern crate day_02_corruption_checksum;
extern crate utils;

use day_02_corruption_checksum::corr_checksum;
use utils::file2str;


fn main() {
    let puzzle_string = file2str("puzzle.txt");
    let checksum = corr_checksum(&puzzle_string);
    println!("The checksum is: {}", checksum);
}
