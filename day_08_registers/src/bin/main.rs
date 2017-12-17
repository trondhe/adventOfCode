extern crate day_08_registers;
extern crate utils;

use day_08_registers::{Processor, ProcessorTrait};
use utils::{file2str, str2linevec};

fn main() {
    let puzzle = file2str("puzzle.txt");
    let puzzle = str2linevec(&puzzle);

    let mut cpu = Processor::new();
    for instruction in puzzle.iter() {
        cpu.process_instruction(instruction)
    }

    let max_value = cpu.highest_value_in_register();
    println!("Highest value in register is: {}", max_value);
}
