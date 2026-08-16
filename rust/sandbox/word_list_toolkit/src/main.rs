// ============================================================
// Word list toolkit
//
// Implement three functions and a main that exercises them.
// Chapters 1-4 only: loops, String/Vec, references, borrowing.
// Iterator chains are optional - plain loops are fine.
//
// ------------------------------------------------------------
// 1. fn longest(words: &Vec<String>) -> &String
//
//    Return a reference to the longest string in `words`.
//    Assume `words` is non-empty.
//
// ------------------------------------------------------------
// 2. fn capitalize_all(words: &mut Vec<String>)
//
//    Uppercase the first character of every word, in place.
//    Returns nothing.
//
//    There is more than one way to do this. Pick one and be
//    able to say why.
//
// ------------------------------------------------------------
// 3. fn add_longer_than(dst: &mut Vec<String>, src: &Vec<String>, min_len: usize)
//
//    Append to `dst` a copy of every string in `src` whose
//    length is at least `min_len`.
//
// ------------------------------------------------------------
// 4. fn main()
//
//    Build a word list, call all three, print between steps.
//
// ------------------------------------------------------------
// 
// ============================================================

fn longest(words: &Vec<String>) -> &String {
    let mut max_len = 0;
    let mut longest_idx = 0;
    for i in 0..words.len() {
        if words[i].len() > max_len {
            max_len = words[i].len();
            longest_idx = i;
        }
    }
    &words[longest_idx]
}

// I didn't know .chars() before implementing that
// Except this, deliberately used only things I'm familiar with
// Claude hinted on some ideas
fn capitalize_all(words: &mut Vec<String>) {
    for word in words {
        let mut idx = 0;
        let mut word_cap = String::new();
        for c in word.chars() {
            if idx == 0 {
                word_cap.push(c.to_ascii_uppercase());
            } else {
                word_cap.push(c);
            }
            idx += 1;
        }
        *word = word_cap;
    }
}

fn add_longer_than(dst: &mut Vec<String>, src: &Vec<String>, min_len: usize) {
    for word in src {
        if word.len() >= min_len {
            dst.push(word.clone());
        }
    }
}

fn main() {
    let words = vec![
        String::from("dragonfruit"),
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
        String::from("watermelon")
    ];
    let extra_words = vec![
        String::from("tomato"),
        String::from("onion"),
        String::from("pepper"),
        String::from("cucumber"),
        String::from("potato"),
    ];

    // finding the longest word
    println!("The longest word is: '{}'", longest(&words));

    // capitalizing the copy of the original vector
    let mut capitalized = words.clone();
    capitalize_all(&mut capitalized);
    println!("Capitalized list of words: {:?}", capitalized);

    // appending words to a copy of the original vector
    let mut extended = words.clone();
    let min_len = 6;
    add_longer_than(&mut extended, &extra_words, min_len);
    println!("Original list plus new words of at least {} characters: {:?}", min_len, extended);

}
