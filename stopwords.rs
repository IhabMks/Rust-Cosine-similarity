use std::fs::File;
use std::io::prelude::*;
pub fn stop_word() -> Vec<String> {
    let mut stop_words_file = File::open("NLTK's s_w").expect("Can't read file.");

    let mut stop_words_string = String::new();
    stop_words_file
        .read_to_string(&mut stop_words_string)
        .expect("Can't read file.");

    return stop_words_string
        .split("\n")
        .map(|v| v.to_owned())
        .collect();
}
