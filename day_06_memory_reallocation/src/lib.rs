use std::collections::HashMap;

pub fn memory_reallocate(vector: Vec<u32>) -> u32 {
    let mut memory = Memory::new();
    memory.init(vector);
    let steps = memory.reallocate();
    steps
}

struct Memory {
    banks: Vec<u32>,
    length: usize,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            banks: Vec::new(),
            length: 0,
        }
    }

    fn init(&mut self, vector: Vec<u32>) {
        self.banks = vector;
        self.length = self.banks.len();
    }

    fn reallocate(&mut self) -> u32 {
        let mut seen_configs = HashMap::new();
        let mut steps = 0;
        while !seen_configs.contains_key(&self.banks) {
            seen_configs.insert(self.banks.clone(), 0);
            self.distribute();
            steps += 1;
        }
        steps
    }

    fn index_high(&self) -> usize {
        let mut highest_val: u32 = 0;
        let mut highest_ind: u32 = 0;
        for (ind, val) in self.banks.iter().enumerate() {
            if val > &highest_val {
                highest_val = *val;
                highest_ind = ind as u32;
            }
        }
        highest_ind as usize
    }

    fn retrieve_high(&mut self) -> (u32, usize) {
        let index = self.index_high();
        let value_high = self.banks[index];
        self.banks[index] = 0;
        (value_high, index)
    }

    fn distribute(&mut self) {
        let (high_val, high_index) = self.retrieve_high();
        let mut index = high_index + 1;
        for _ in 0..high_val {
            if index >= self.length {
                index = 0;
            }
            self.banks[index] += 1;
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mem(vec: Vec<u32>) -> Memory {
        let mut memory = Memory::new();
        memory.init(vec);
        memory
    }

    #[test]
    fn can_create_memory_of_vec() {
        let length: usize = 10;
        let vector: Vec<u32> = vec![0; length];
        let mem = create_mem(vector);
        assert_eq!(mem.length, length);
    }

    #[test]
    fn can_find_highest_valueindex() {
        let vector: Vec<u32> = vec![0, 1, 5, 1, 5];
        let mem = create_mem(vector);
        let index = mem.index_high();
        assert_eq!(index, 2);
    }

    #[test]
    fn can_retrive_highest_index_value() {
        let vector = vec![0, 1, 2, 10, 5];
        let mut mem = create_mem(vector);
        let (highest_value, _) = mem.retrieve_high();
        assert_eq!(highest_value, 10);
        assert_eq!(mem.banks[3], 0);
    }

    #[test]
    fn can_distribute_highest_val_basic() {
        let mut mem = create_mem(vec![2, 0, 0]);
        mem.distribute();
        assert_eq!(mem.banks, vec![0, 1, 1]);
    }

    #[test]
    fn can_distribute_highest_val_intermediate() {
        let mut mem = create_mem(vec![0, 4, 5, 3, 1]);
        mem.distribute();
        assert_eq!(mem.banks, vec![1, 5, 1, 4, 2]);
    }

    #[test]
    fn steps_to_earlier_config() {
        let mut mem = create_mem(vec![0, 2, 7, 0]);
        let steps = mem.reallocate();
        assert_eq!(steps, 5);
    }
}
