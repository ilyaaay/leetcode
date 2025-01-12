use std::collections::HashMap;

fn main() {}

fn _two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        if let Some(&value) = map.get(&(target - nums[i])) {
            return Vec::from([value, i as i32]);
        }

        map.insert(nums[i], i as i32);
    }

    return Vec::new();
}

#[test]
fn test() {
    assert_eq!(_two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
