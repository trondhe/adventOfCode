extern crate day_03_spiral_memory;
use day_03_spiral_memory::{manhattan_distance, spiral_memory_value_location};

fn main() {
    let find_val_ind = 277678;
    let (row, col, center) = spiral_memory_value_location(find_val_ind);
    let manhattan_dist = manhattan_distance(row, col, center);
    println!("Manhattan distance is: {}", manhattan_dist);
}
