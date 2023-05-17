use std::cmp::{max, min};

fn main() {
    let data: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("{}", max_area(&data));
}

fn max_area(height: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    let (mut left, mut right): (i32, i32) = (0, height.len() as i32 - 1);
    while left < right {
        let area: i32 = (right - left) * min(height[left as usize], height[right as usize]);
        result = max(result, area);
        if height[left as usize] < height[right as usize] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    result
}
