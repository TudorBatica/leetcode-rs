// https://leetcode.com/problems/valid-parentheses/

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in s.chars() {
        match stack.last() {
            None => { stack.push(char) }
            Some(c) => {
                if is_open_bracket(&char) {
                    stack.push(char);
                    continue;
                }
                if get_corresponding_open_bracket(&char) == *c {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
    }

    return stack.len() == 0;
}

fn is_open_bracket(char: &char) -> bool {
    return *char == '(' || *char == '[' || *char == '{';
}

fn get_corresponding_open_bracket(char: &char) -> char {
    if *char == ']' {
        return '[';
    } else if *char == ')' {
        return '(';
    } else if *char == '}' {
        return '{';
    } else {
        panic!("Invalid argument");
    }
}

#[cfg(test)]
mod tests {
    use crate::valid_parentheses_20::is_valid;

    #[test]
    pub fn test_1() {
        assert_eq!(is_valid("()[]{}".to_string()), true)
    }

    #[test]
    pub fn test_2() {
        assert_eq!(is_valid("([)]".to_string()), false)
    }

    #[test]
    pub fn test_3() {
        assert_eq!(is_valid("{()}".to_string()), true)
    }
}
