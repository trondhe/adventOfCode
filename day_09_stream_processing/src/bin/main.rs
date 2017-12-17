extern crate day_09_stream_processing;
extern crate utils;

use day_09_stream_processing::process_stream;
use utils::file2str;

fn main() {
    let puzzle = file2str("puzzle.txt");
    let sum = process_stream(puzzle);

    println!("The sum is: {}", sum);
}
