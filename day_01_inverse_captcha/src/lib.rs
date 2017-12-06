extern crate utils;
use utils::str2vec_u32;

pub fn inv_captcha(vec: Vec<u32>) -> u32 {
    let vec_len = vec.len();
    if vec_len <= 1 {
        return 0;
    }
    inv_captcha_iterate_vec(&vec, &vec_len)
}

fn inv_captcha_iterate_vec(vec: &Vec<u32>, vec_len: &usize) -> u32 {
    let mut sum: u32 = 0;
    let vec_end = vec_len.wrapping_sub(1);
    // Compare vector iterativly
    for index in 0..vec_end {
        if index != vec_end {
            add_if_same(&vec[index], &vec[index + 1], &mut sum);
        }
    }
    // Compare last with first
    add_if_same(&vec[vec_end], &vec[0], &mut sum);
    sum
}

fn add_if_same(val_to_add: &u32, val_to_comp: &u32, sum: &mut u32) {
    if val_to_add == val_to_comp {
        *sum += *val_to_add;
    }
}


#[cfg(test)]
mod inv_captcha_test {
    use super::*;


    fn ic_assert(input: &str, expected: u32) {
        let input = str2vec_u32(input);
        let answer = inv_captcha(input);
        assert_eq!(answer, expected);
    }

    #[test]
    fn given_nothing_return_0() {
        ic_assert("", 0)
    }

    #[test]
    fn given_1_return_0() {
        ic_assert("1", 0)
    }

    #[test]
    fn given_11_return_2() {
        ic_assert("11", 2)
    }

    #[test]
    fn given_112_return_1() {
        ic_assert("112", 1)
    }

    #[test]
    fn given_1122_return_3() {
        ic_assert("1122", 3)
    }

    #[test]
    fn given_1111_return_4() {
        ic_assert("1111", 4)
    }

    #[test]
    fn given_1234_return_0() {
        ic_assert("1234", 0)
    }

    #[test]
    fn given_91212129_return_9() {
        ic_assert("91212129", 9)
    }
}
