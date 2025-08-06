use std::collections::HashMap;

fn main() {}

#[doc = "time complexity = O(n2), space complexity = O(1)"]
fn two_sum_first(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    Vec::new()
}

#[doc = "time complexity = O(n), space complexity = O(n)"]
fn two_sum_second(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let map = (0..nums.len())
        .map(|x| (nums[x], x))
        .collect::<HashMap<i32, usize>>();

    for i in 0..nums.len() {
        if let Some(&value) = map.get(&(target - nums[i]))
            && value != i
        {
            return vec![i as i32, value as i32];
        }
    }

    Vec::new()
}

#[doc = "time complexity = O(n2), space complexity = O(1)"]
fn two_sum_three(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indeces_map = HashMap::new();

    for (idx, x) in nums.iter().enumerate() {
        if let Some(&value_idx) = indeces_map.get(&(target - x)) {
            return vec![value_idx as i32, idx as i32];
        }

        indeces_map.insert(x, idx);
    }

    Vec::new()
}

#[test]
fn test_two_sum_first() {
    let input = vec![2, 7, 11, 15];
    let actual = two_sum_first(input, 9);
    let expected = [1, 0];

    assert_eq!(actual, expected);

    let input = vec![3, 2, 4];
    let actual = two_sum_first(input, 6);
    let expected = [2, 1];

    assert_eq!(actual, expected);

    let input = vec![3, 3];
    let actual = two_sum_first(input, 6);
    let expected = [1, 0];

    assert_eq!(actual, expected);
}

#[test]
fn test_two_sum_second() {
    let input = vec![2, 7, 11, 15];
    let actual = two_sum_second(input, 9);
    let expected = [0, 1];

    assert_eq!(actual, expected);

    let input = vec![3, 2, 4];
    let actual = two_sum_second(input, 6);
    let expected = [1, 2];

    assert_eq!(actual, expected);

    let input = vec![3, 3];
    let actual = two_sum_second(input, 6);
    let expected = [0, 1];

    assert_eq!(actual, expected);
}

#[test]
fn test_two_sum_three() {
    let input = vec![2, 7, 11, 15];
    let actual = two_sum_three(input, 9);
    let expected = [0, 1];

    assert_eq!(actual, expected);

    let input = vec![3, 2, 4];
    let actual = two_sum_three(input, 6);
    let expected = [1, 2];

    assert_eq!(actual, expected);

    let input = vec![3, 3];
    let actual = two_sum_three(input, 6);
    let expected = [0, 1];

    assert_eq!(actual, expected);
}
