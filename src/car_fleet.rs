// https://leetcode.com/problems/car-fleet/

use std::cmp::min;

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut pos_and_speed: Vec<(&i32, &i32)> = position.iter().zip(speed.iter()).collect();
    pos_and_speed.sort_by_key(|(pos, _speed)| std::cmp::Reverse(*pos));

    let (_, min_speed) = pos_and_speed.iter().min_by_key(|(_pos, speed)| **speed).unwrap();
    let max_hours_spent_on_road = (target / **min_speed) as usize;

    let mut position_by_timestamp: Vec<i32> = vec![-1; max_hours_spent_on_road];
    let mut fleets = pos_and_speed.len() as i32;

    for (car_pos, car_speed) in pos_and_speed {
        let mut timestamp = 0;
        let mut position;
        loop {
            position = *car_pos + car_speed * timestamp;
            match position_by_timestamp.get(timestamp as usize) {
                Some(value) => {
                    if *value == -1 || *value > position {
                        position_by_timestamp[timestamp as usize] = position;
                    } else {
                        fleets -= 1;
                        break;
                    }
                }
                None => {}
            }
            if position >= target {
                break;
            }
            timestamp += 1;
        }
    }

    return fleets;
}

#[cfg(test)]
mod tests {
    use crate::car_fleet::car_fleet;

    #[test]
    pub fn test_1() {
        assert_eq!(car_fleet(10, vec![8,3,7,4,6,5], vec![4,4,4,4,4,4]), 6)
    }

}

