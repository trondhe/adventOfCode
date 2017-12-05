pub fn manhattan_distance(row: usize, col: usize, center: usize) -> usize {
    let distance = (row as i32 - center as i32).abs() + (col as i32 - center as i32).abs();
    distance as usize
}

pub fn spiral_memory_value_location(value: u32) -> (usize, usize, usize) {
    let mut column_index = 0;
    let mut row_index = 0;
    let (smem, center) = spiral_memory(value);
    for (index, row) in smem.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ind| ind == value) {
            column_index = c;
            row_index = index;
            break;
        }
    }
    (row_index, column_index, center)
}

fn spiral_memory(min_count: u32) -> (Vec<Vec<u32>>, usize) {
    let (min_size, center) = odd_size_grid(min_count);
    let mut grid = create_grid(min_size);
    spiral_memory_init(&mut grid, min_size, center);
    (grid, center)
}

fn spiral_memory_init(grid: &mut Vec<Vec<u32>>, min_size: u32, c: usize) {
    if min_size == 1 {
        grid[c][c] = 1;
        return;
    }

    let mut count = 1;
    let mut steps = 1;
    let mut dir = 1 as i32;
    let mut x = c as i32;
    let mut y = c as i32;

    loop {
        for _ in 0..steps {
            grid[y as usize][x as usize] = count;
            x += dir;
            count += 1;
        }
        if steps >= min_size {
            break;
        }
        dir *= -1;
        for _ in 0..steps {
            grid[y as usize][x as usize] = count;
            y += dir;
            count += 1;
        }
        steps += 1;
    }
}

fn create_grid(size: u32) -> Vec<Vec<u32>> {
    let grid = vec![vec![0; size as usize]; size as usize];
    grid
}

fn odd_size_grid(input: u32) -> (u32, usize) {
    if input == 1 {
        return (1, 0);
    }
    let min_size = min_grid_size(&input);
    let center = center_of_grid(&min_size);
    (min_size, center)
}


fn min_grid_size(input: &u32) -> u32 {
    let input = *input as u32;
    if input == 1 {
        return 1;
    }
    let mut min_size = (input as f64).sqrt() as u32;
    while (min_size.pow(2) < input) || (min_size % 2 == 0) {
        min_size += 1;
    }
    min_size
}

fn center_of_grid(input: &u32) -> usize {
    let center = (*input / 2) as usize;
    center
}

#[cfg(test)]
mod manhattan_dist_test {
    use super::*;

    #[test]
    fn get_index_of_number_in_smem() {
        let find_val_ind = 8;
        let (row, col, _) = spiral_memory_value_location(find_val_ind);
        assert_eq!(row, 2);
        assert_eq!(col, 1);
    }

    #[test]
    fn can_calculate_manhattan_distance_from_1() {
        let find_val_ind = 1;
        let (row, col, center) = spiral_memory_value_location(find_val_ind);
        let manhattan_dist = manhattan_distance(row, col, center);
        assert_eq!(manhattan_dist, 0);
    }

    #[test]
    fn can_calculate_manhattan_distance_from_12() {
        let find_val_ind = 12;
        let (row, col, center) = spiral_memory_value_location(find_val_ind);
        let manhattan_dist = manhattan_distance(row, col, center);
        assert_eq!(manhattan_dist, 3);
    }

    #[test]
    fn can_calculate_manhattan_distance_from_1024() {
        let find_val_ind = 1024;
        let (row, col, center) = spiral_memory_value_location(find_val_ind);
        let manhattan_dist = manhattan_distance(row, col, center);
        assert_eq!(manhattan_dist, 31);
    }
}


#[cfg(test)]
mod spiral_memory_test {
    use super::*;

    #[test]
    fn can_create_spiral_memory_1x1() {
        let (s_mem, _) = spiral_memory(1);
        assert_eq!(s_mem[0][0], 1);
    }

    #[test]
    fn can_create_spiral_memory_3x3() {
        let (s_mem, _) = spiral_memory(9);
        assert_eq!(s_mem[1][1], 1);
        assert_eq!(s_mem[1][2], 2);
        assert_eq!(s_mem[0][2], 3);
        assert_eq!(s_mem[0][1], 4);
    }
}

#[cfg(test)]
mod grid_size_test {
    use super::*;

    #[test]
    fn given_1_return_1_0() {
        let (min_size, center) = odd_size_grid(1);
        assert_eq!(min_size, 1);
        assert_eq!(center, 0);
    }

    #[test]
    fn given_2_return_3_1() {
        let (min_size, center) = odd_size_grid(2);
        assert_eq!(min_size, 3);
        assert_eq!(center, 1);
    }

    #[test]
    fn given_9_return_3_1() {
        let (min_size, center) = odd_size_grid(9);
        assert_eq!(min_size, 3);
        assert_eq!(center, 1);
    }

    #[test]
    fn given_25_return_5_2() {
        let (min_size, center) = odd_size_grid(25);
        assert_eq!(min_size, 5);
        assert_eq!(center, 2);
    }

    #[test]
    fn given_26_return_7_3() {
        let (min_size, center) = odd_size_grid(26);
        assert_eq!(min_size, 7);
        assert_eq!(center, 3);
    }
}
