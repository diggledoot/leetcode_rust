fn main() {
    let data: i32 = 121;
    println!("{:?}", is_palindrome(data))
}

fn is_palindrome(x: i32) -> bool {
    let num_string: String = x.clone().to_string();

    if num_string.len() == 1 {
        return true;
    }

    let is_signed: bool = num_string.contains('-') || num_string.contains('+');
    if is_signed {
        return false;
    }

    let reversed_string: String = num_string.chars().rev().collect();
    if reversed_string.starts_with('0') {
        return false;
    }

    if num_string.eq(&reversed_string) {
        return true;
    }

    false
}
