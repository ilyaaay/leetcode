fn main() {
    let input = Vec::from([1, 3, 8, 48, 10]);

    let x = longest_nice_subarray(input);

    println!("{x:?}");
}

fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut bitwise_zero_vec = Vec::new();

    for i in 0..len - 1 {
        let x = nums[i] & nums[i + 1];

        bitwise_zero_vec.push(x);
    }

    println!("{:?}", bitwise_zero_vec);

    0
}

#[test]
fn test() {
    let nums = Vec::from([1, 3, 8, 48, 10]);

    assert_eq!(longest_nice_subarray(nums), 3);
}
