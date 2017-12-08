pub fn high_entropy_pass(input: &str) -> bool {
    let string_array: Vec<&str> = input.split(' ').collect();
    for (ind1, string1) in string_array.iter().enumerate() {
        for (ind2, string2) in string_array.iter().enumerate() {
            if ind1 == ind2 {
                continue;
            }
            if string1 == string2 {
                return false;
            }
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_passphrase() {
        assert!(high_entropy_pass("a"));
        assert!(!high_entropy_pass("a a"));
        assert!(high_entropy_pass("a b"));
        assert!(high_entropy_pass("aa bb cc dd ee"));
        assert!(!high_entropy_pass("aa bb cc dd aa"));
        assert!(high_entropy_pass("aa bb cc dd aaa"));
    }
}
