extern crate day_05_twisty_trampoline_maze;
extern crate utils;

use day_05_twisty_trampoline_maze::maze_run;
use utils::{file2str, str2linevec};



fn main() {
    let puzzle = file2str("puzzle.txt");
    let puzzle = str2linevec(&puzzle);
    let mut puzzle_vec: Vec<i32> = Vec::new();
    for val in puzzle {
        puzzle_vec.push(val.parse().unwrap());
    }

    let steps = maze_run(&mut puzzle_vec);
    println!("Steps to exit is: {}", steps);
}
