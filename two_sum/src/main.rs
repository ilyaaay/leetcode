use std::collections::HashMap;

fn main() {
    let input = vec![3, 2, 4];
    let target = 6;

    println!("{:#?}", two_sum(input, target));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let x = target - nums[i];

        if let Some(value) = map.get(&x) {
            return Vec::from([*value, i as i32]);
        }

        map.insert(nums[i], i as i32);
    }

    return Vec::new();
}

#[test]
fn test() {
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
