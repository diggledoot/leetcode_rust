fn main() {
    let mut data: Vec<i32> = vec![3, 2, 2, 3];
    let target: i32 = 3;
    println!("{} {:?}", remove_element(&mut data, target), data);
}
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while j < nums.len() {
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
        }
        j += 1;
    }
    nums.truncate(i);
    i as i32
}
