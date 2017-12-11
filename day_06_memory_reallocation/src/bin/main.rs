extern crate day_06_memory_reallocation;
use day_06_memory_reallocation::memory_reallocate;

fn main() {
    let puzzle = vec![4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let steps = memory_reallocate(puzzle);
    println!("Steps to seen config: {}", steps);
}
