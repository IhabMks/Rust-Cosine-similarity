//use tokenizers::tokenizer::{Result, Tokenizer};
use std::collections::HashMap;
mod stopwords;
mod questions;
extern crate time;
use time::PreciseTime;
fn main() {
    let mut my_hash = HashMap::new();
    let stop_word: Vec<String> = stopwords::stop_word();
    let questions: Vec<String> = questions::questions();
    let mut main_count:u8 = 0;
    for (index, main_question) in questions.iter().enumerate() {
        let start = PreciseTime::now();
        //println!("index : {} -> {}", index, main_question);
        let mut similarities: Vec<i32> = Vec::new();
        let mut b: Vec<String> = questions.to_vec();
        let x_vec: Vec<String> = main_question.split(" ").map(|v| String::from(v)).collect(); // basic method of tokenizer based on split_whitespace()
        //let x_vec: Vec<String> = load(&main_question).unwrap();
        b.remove(index);
        let mut count: u32= 1;
        for question in b {
            let mut _l1: Vec<u16> = Vec::new();
            let mut _l2: Vec<u16> = Vec::new();
            let  y_vec: Vec<String> = question.split(" ").map(|v| String::from(v)).collect();
            //let y_vec: Vec<String> = load(&question).unwrap();

            if x_vec.len() == 0 || y_vec.len() == 0 {
                continue;
            }
            let mut x_set: Vec<&str> = vec![];
            let mut y_set: Vec<&str> = vec![];
            for word in &x_vec {
                if !stop_word.contains(&word) {
                    x_set.push(&word);
                }
            }
            for word in &y_vec {
                if !stop_word.contains(&word) {
                    y_set.push(&word);
                }
            }
            //let mut rvector: Vec<&str> = [x_set, y_set].concat();
            let mut rvector: Vec<&str> = x_set.iter().copied().chain(y_set.iter().copied()).collect();
            rvector.sort();
            rvector.dedup();
            
            for word in &rvector {
                if x_set.contains(&word) {
                    _l1.push(1);
                } else {_l1.push(0)}
                if y_set.contains(&word) {
                    _l2.push(1);
                } else {_l2.push(0)}
            }
            
            let mut c: u16 = 0;
            for i in 0..(rvector.len()){
                c+= _l1[i]*_l2[i];
            }
            
            let cosine: f32 = (c as f32) / f32::powf((_l1.iter().sum::<u16>() * _l2.iter().sum::<u16>()) as f32, 0.5);
            if cosine > 0.6{
                similarities.push(questions.iter().position(|r| r == &question).unwrap() as i32);
            }
            println!("Index: {} -> iter nb: {}",index, count);
            count+=1;
        }
        let end = PreciseTime::now();
        println!("Index: {} lasted {} seconds.", index, start.to(end));
        my_hash.insert(index, similarities);
        if main_count == 10{break}
        main_count +=1;
    }
    println!("hash {:?}",my_hash);
}

//fn load(text: &String) -> Result<Vec<String>> {
//    let s_slice: &str = &text[..];  // take a full slice of the string
//    //let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?; // download the tokenizer if not available offline
//    let tokenizer = Tokenizer::from_file("./tokenizer.json"); // import the tokenizer in case it's already saved
//                                                              //tokenizer.save("tokenizer.json", false)?; // save the tokenizer to a json file
//    let encoding = tokenizer.unwrap().encode(s_slice, false)?; // if downloaded and not imported offline, the tokenizer return a result, so unwrap() it first then encode.
//                                                            //println!("{:?}", encoding.get_tokens()); // print test
//    Ok(encoding.get_tokens().to_vec())
//}