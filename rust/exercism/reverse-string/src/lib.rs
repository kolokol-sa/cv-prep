pub fn reverse(input: &str) -> String {
    let mut output = String::new(); 
    for c in input.chars() {
        let mut helper = String::new();
        helper.push(c);
        helper.push_str(&output);
        output = helper.clone();
    }
    output
}
