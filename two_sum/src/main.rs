use std::collections::HashMap;

fn main() {}

// O(n) space complexity
fn two_sum_best_by_space_complexity(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx, x) in nums.iter().enumerate() {
        for (idx_two, y) in nums.iter().enumerate() {
            if x != y && x + y == target {
                return Vec::from([idx as i32, idx_two as i32]);
            }
        }
    }

    Vec::new()
}

// O(n) time complexity
fn two_sum_best_by_time_complexity(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (idx, x) in nums.into_iter().enumerate() {
        if let Some(&a) = map.get(&(target - x)) {
            return Vec::from([a as i32, idx as i32]);
        }

        map.insert(x, idx);
    }

    Vec::new()
}

#[test]
fn test() {
    assert_eq!(
        two_sum_best_by_time_complexity(vec![1, 2, 3], 5),
        vec![1, 2]
    );

    assert_eq!(
        two_sum_best_by_time_complexity(vec![3, 2, 4], 6),
        vec![1, 2]
    )
}
