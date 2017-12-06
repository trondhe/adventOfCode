extern crate utils;
use utils::str2vecinception;


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
        let input = "1 2 3\r\n4 5 6\r\n7 8 9";
        let result = corr_checksum(input);
        let expected = 6;
        assert_eq!(result, expected)
    }
}
