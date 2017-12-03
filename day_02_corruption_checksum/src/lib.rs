pub fn corr_checksum(input_string: &str) -> u32 {
    let array = str2vecinception(input_string);
    let sum = get_difference_of_array(array);
    sum
}

fn get_difference_of_array(array: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in array.iter() {
        sum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    sum
}

pub fn str2vecinception(input: &str) -> Vec<Vec<u32>> {
    let lines = str2linevec(&input);
    let vector = lines2vecvec(lines);
    vector
}

fn str2linevec(string: &str) -> Vec<&str> {
    let lines: Vec<&str> = string.split("\r\n").collect();
    lines
}

fn lines2vecvec(lines: Vec<&str>) -> Vec<Vec<u32>> {
    let mut vector: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let row: Vec<u32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        vector.push(row);
    }
    vector
}

#[cfg(test)]
mod corruption_checksum_test {
    use super::*;

    #[test]
    fn given_single_return_0() {
        let input = "1";
        let result = corr_checksum(input);
        let expected = 0;
        assert_eq!(result, expected)
    }

    #[test]
    fn given_1_2_return_sum() {
        let input = "1 2";
        let result = corr_checksum(input);
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn given_1_3_return_sum() {
        let input = "1 3";
        let result = corr_checksum(input);
        let expected = 2;
        assert_eq!(result, expected)
    }

    #[test]
    fn given_3x3_return_sum() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let result = corr_checksum(input);
        let expected = 6;
        assert_eq!(result, expected)
    }

}


#[cfg(test)]
mod str2vecinception_test {
    use super::*;
    // Tab seperated values, newline seperated rows

    #[test]
    fn assert_1x1() {
        let string = "525";
        let result = str2vecinception(string);
        let expected = [[525]];
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_1x2() {
        let string = "111 222\n333 444";
        let result = str2vecinception(string);
        let expected = [[111, 222], [333, 444]];
        assert_eq!(result, expected);
    }

    fn assert_3x3() {
        let string = "1 2\n3\n4 5";
        let result = str2vecinception(string);
        let expected = vec![vec![1, 2], vec![3], vec![4, 5]];
        assert_eq!(result, expected);
    }
}
