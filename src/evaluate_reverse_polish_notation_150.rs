// https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        match &token.parse::<i32>() {
            Ok(number) => {
                stack.push(*number);
            }
            Err(_) => {
                let operand2 = stack.pop().expect("Should be valid as per Leetcode description");
                let operand1 = stack.pop().expect("Should be valid as per Leetcode description");
                stack.push(apply_operator(operand1, operand2, &token));
            }
        }
    }

    return *stack.last().expect("Should be valid as per Leetcode description");
}

fn apply_operator(operand1: i32, operand2: i32, operator: &str) -> i32 {
    if operator == "+" {
        return operand1 + operand2;
    } else if operator == "-" {
        return operand1 - operand2;
    } else if operator == "*" {
        return operand1 * operand2;
    } else if operator == "/" {
        return operand1 / operand2;
    } else {
        panic!("Invalid operator provided");
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate_reverse_polish_notation_150::eval_rpn;

    #[test]
    pub fn test_1() {
        assert_eq!(eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]), 9)
    }
}
