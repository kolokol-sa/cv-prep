// Read a hardcoded string (no file I/O — that's chapter 9 territory with Result) and produce:

// - Total character count and byte count, reported separately.
// - Word frequencies, sorted by count descending, printing the top five.
// - The first index at which each distinct character appears.

// Write it as a single binary, functions only — no modules.

// Decisions to make deliberately:

// - Whether the frequency map keys are String or &str, and whether that choice is forced by what the function returns.
// - Whether any of these needs the entry API or whether insert suffices.
// - How you get from a HashMap to something sorted, since maps have no order.

// Test string, with a non-ASCII word so the byte-versus-char distinction shows up:
// let text = "the café is the place the crowd meets café at noon";

use std::collections::HashMap;

fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn byte_count(s: &str) -> usize {
    s.len()
}

fn word_frequency(s: &str) -> Vec<(String, usize)> {
    let mut map = HashMap::new();
    for word in s.split_whitespace() {
        let mut count = map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }
    let mut vec = map.into_iter().collect::<Vec<(String, usize)>>();
    vec.sort_by_key(|x| x.1);
    vec.reverse();
    vec
}

fn char_first_indices(s: &str) -> Vec<(char, usize)> {
    let mut map = HashMap::new();
    for (i, ch) in s.chars().enumerate() {
        map.entry(ch).or_insert(i);
    }
    let mut vec = map.into_iter().collect::<Vec<(char, usize)>>();
    vec.sort_by_key(|ch| ch.0);
    vec
}

fn main() {
    let text = "the café opens at noon and the café closes at dusk \
            the crowd meets the crowd leaves the naïve barista \
            counts the cups and the cups keep coming at noon";
    println!("The input string is \"{}\"", text);

    println!("\nString statistics:");
    // character and byte count
    println!("- Total character count: {} characters", char_count(&text));
    println!("- Total byte count: {} bytes", byte_count(&text));

    // top 5 most frequent words
    println!("\nThe 5 most frequent words are:");
    let word_freq = word_frequency(text);
    let word_freq_len = word_freq.len();
    let end = if word_freq_len >= 5 { 5 } else { word_freq_len };
    for (word, freq) in word_freq[0..end].iter() {
        println!("{} - {}", word, freq);
    }

    // indices of the first character appearances
    println!("\nWhen each character first appears:");
    let char_indices = char_first_indices(text);
    for (ch, i) in char_indices.iter() {
        println!("'{}' - [{}]", ch, i);
    }
}
