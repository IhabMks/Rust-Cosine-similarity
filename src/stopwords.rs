use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn stop_word() -> HashSet<String> {
    let mut stop_words_file = File::open("./resources/NLTK's s_w.txt").expect("Can't read file.");

    let mut stop_words_string = String::new();
    stop_words_file
        .read_to_string(&mut stop_words_string)
        .expect("Can't read file.");

    return stop_words_string
        .split("\n")
        .map(|s| s.to_string())
        .collect();
}
