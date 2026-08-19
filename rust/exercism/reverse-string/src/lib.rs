// Problem: reverse a given string
// There are two valid solutions here, one with a loop and another with collect
// Both are equally good and worth keeping: the first is easier to read, the second is more concise
// The implementation doesn't account for combining characters

pub fn reverse(input: &str) -> String {
    let mut output = String::with_capacity(input.len()); 
    for c in input.chars().rev() {
        output.push(c);
    }
    output
}

pub fn reverse_collect(input: &str) -> String {
    input.chars().rev().collect()
}