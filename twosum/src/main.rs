use std::collections::HashMap;

fn main() {
    let data = vec![2, 7, 11, 15];
    println!("{:?}", twosum(data, 9));
}

fn twosum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();
    for (idx, value) in nums.iter().enumerate() {
        let diff: i32 = target - value;
        if result.contains_key(&diff) {
            return vec![result[&diff], idx as i32];
        }
        result.insert(*value, idx as i32);
    }
    unreachable!()
}
