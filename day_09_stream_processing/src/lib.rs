pub fn process_stream(string: String) -> u32 {
    let mut string_iterator = string.chars();
    let mut sum = 0;
    let mut level = 0;
    let mut garbage_mode = false;

    let mut contains = string_iterator.next();
    while contains != None {
        let c = contains.unwrap();
        if c == '<' {
            garbage_mode = true;
        }
        if garbage_mode {
            match c {
                '!' => {
                    string_iterator.next();
                }
                '>' => garbage_mode = false,
                _ => (),
            }
        } else {
            match c {
                '{' => level += 1,
                '}' => {
                    sum += level;
                    level -= 1;
                }
                _ => (),
            }
        }
        contains = string_iterator.next();
    }
    sum
}

#[cfg(test)]
mod stream_process_test {
    use super::*;

    fn stream_gives_sum(string: &str, sum: u32) {
        let string = String::from(string);
        assert_eq!(process_stream(string), sum);
    }

    #[test]
    fn test_1() {
        stream_gives_sum("", 0);
    }

    #[test]
    fn test_2() {
        stream_gives_sum("{{{}}}", 6);
    }

    #[test]
    fn test_3() {
        stream_gives_sum("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9);
    }

    #[test]
    fn test_4() {
        stream_gives_sum("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9);
    }

    #[test]
    fn test_5() {
        stream_gives_sum("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3);
    }
}
