pub fn is_armstrong_number(num: u32) -> bool {
    // this long line converts number into a string, breaks it down into digits and stores them as u32 in a vector
    let digits: Vec<u32> = num.to_string().chars().map(|x| (x as u32) - 48).collect();
    if num == digits.iter().map(|x| x.pow(digits.len() as u32)).sum() {
        true
    } else {
        false
    }
}