use std::fs::File;
use std::io::prelude::*;

pub fn questions() -> Vec<String> {
    let mut questions_file = File::open("./resources/questions.txt").expect("Can't read file.");

    let mut questions_string = String::new();
    questions_file
        .read_to_string(&mut questions_string)
        .expect("Can't read file.");

    return questions_string.split("\n").map(|s| s.to_string()).collect();
}
