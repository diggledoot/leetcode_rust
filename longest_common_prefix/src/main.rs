fn main() {
    let data: Vec<String> = vec![
        String::from("flower"),
        String::from("flower"),
        String::from("flower"),
    ];
    println!("{}", longest_common_prefix(data))
}

fn longest_common_prefix(strings: Vec<String>) -> String {
    if strings.len() <= 1 {
        return strings[0].clone();
    }
    let mut result: String = "".into();
    let first_string = &strings[0];
    for (idx, char) in first_string.chars().enumerate() {
        for _string in strings.iter() {
            if idx == _string.len()
                || _string.clone().chars().nth(idx) != first_string.chars().nth(idx)
            {
                return result;
            }
        }
        result.push(char);
    }
    result
}
