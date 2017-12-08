pub fn maze_run(maze: &mut Vec<i32>) -> u32 {
    let mut jumps: u32 = 0;
    let mut index: i32 = 0;
    let mut index_prev: i32;

    while !out_of_bounds(&maze, &index) {
        index_prev = index;
        index += maze[index as usize];
        maze[index_prev as usize] += 1;
        jumps += 1;
    }
    jumps
}

fn out_of_bounds(vec: &Vec<i32>, index: &i32) -> bool {
    let vec_length = vec.len() as i32;
    if *index > vec_length - 1 || *index < 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn maze_run_asserter(maze: Vec<i32>, expect_steps: u32) {
        let mut maze = maze.clone();
        let steps = maze_run(&mut maze);
        assert_eq!(expect_steps, steps);
    }

    #[test]
    fn can_escape_maze() {
        maze_run_asserter(vec![1], 1);
        maze_run_asserter(vec![0], 2);
        maze_run_asserter(vec![0, 3, 0, 1, -3], 5);
    }

    #[test]
    fn bound_checker_test() {
        let vec = vec![0; 1];
        assert!(!out_of_bounds(&vec, &0));
        assert!(out_of_bounds(&vec, &1));
        assert!(out_of_bounds(&vec, &-1));

        let vec = vec![0; 10];
        assert!(!out_of_bounds(&vec, &5));
        assert!(out_of_bounds(&vec, &10));
        assert!(out_of_bounds(&vec, &-10));
    }

}
