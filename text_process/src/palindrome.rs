pub fn check_palindrome(input: &str) -> bool {
    if input.len() == 0 {
        return true;
    }

    let mut last = input.len() - 1;
    let mut first = 0;

    let my_vec = input.as_bytes().to_owned();

    while first < last {
        if my_vec[fistt] != my_vec[last] {
            return false;
        }
        
        first += 1;
        last -= 1;
    }

    return true;
}

