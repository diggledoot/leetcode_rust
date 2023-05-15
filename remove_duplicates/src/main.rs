fn main() {
    let mut data: Vec<i32> = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut data));
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut i: i32 = 1;
    for idx in 1..nums.len() {
        if nums[i as usize - 1] != nums[idx] {
            nums.swap(i as usize, idx);
            i += 1;
        }
    }
    i
}
