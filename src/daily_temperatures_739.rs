// https://leetcode.com/problems/daily-temperatures/

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut stack: Vec<(i32, usize)> = Vec::new();

    for (index, temp) in temperatures.iter().rev().enumerate() {
        loop {
            match &stack.last() {
                None => {
                    res.push(0);
                    stack.push((*temp, index));
                    break;
                }
                Some((tmp, i)) => {
                    if temp.ge(tmp) {
                        stack.pop();
                    } else {
                        res.push((index - i) as i32);
                        stack.push((*temp, index));
                        break;
                    }
                }
            }
        }
    }

    res.reverse();

    return res;
}

#[cfg(test)]
mod tests {
    use crate::daily_temperatures_739::daily_temperatures;

    #[test]
    pub fn test_1() {
        assert_eq!(daily_temperatures(vec![73,74,75,71,69,72,76,73]), vec![1,1,4,2,1,1,0,0]);
    }


}
