#![allow(dead_code)]

fn main() {
    let mut vec = &mut vec![2, 0, 2, 1, 1, 0];

    second_solution::sort_colors(&mut vec);

    println!("{:?}", vec);
}

mod first_solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut x, mut y, mut z) = (0, 0, 0);

        for i in 0..nums.len() {
            match nums[i] {
                0 => x += 1,
                1 => y += 1,
                _ => z += 1,
            }
        }

        let mut index = 0;

        for _ in 0..x {
            nums[index] = 0;
            index += 1;
        }

        for _ in 0..y {
            nums[index] = 1;
            index += 1;
        }

        for _ in 0..z {
            nums[index] = 2;
            index += 1;
        }
    }

    #[test]
    fn test1() {
        let mut actual = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];

        sort_colors(&mut actual);

        assert_eq!(actual, expected);
    }
}

mod second_solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut index = 0;

        while left < right {
            let mut mid = (left + right) / 2;
        }
    }

    #[test]
    fn test2() {}
}
