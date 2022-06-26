use std::{
    collections::{BTreeMap, HashSet},
    fs,
    usize::MAX,
};
extern crate rayon;
use rayon::prelude::*;
mod questions;
mod stopwords;

fn cosine_similarity(
    cutoff: f32,
    full_set: &Vec<HashSet<String>>,
    x_index: usize,
    x_set: &HashSet<String>,
) -> (usize, Vec<usize>) {
    let l1sum = x_set.len();
    let mut similarities: Vec<usize> = Vec::new();
    // let mut count: u32 = 1;
    if !x_set.is_empty() {
        for (y_index, y_set) in full_set.iter().enumerate() {
            if y_index == x_index || y_set.is_empty() {
                continue;
            }

            let xy_set: HashSet<&String> = x_set.intersection(y_set).collect();

            let l2sum = y_set.len();
            let c = xy_set.len();

            let cosine = (c as f32) / ((l1sum * l2sum) as f32).sqrt();
            if cosine > cutoff {
                similarities.push(y_index);
            }
            // println!("Index: {} -> iter nb: {}",index, count);
            // count += 1;
        }
    }
    (x_index, similarities)
}

const MAIN_LIMIT: usize = MAX;
const COSINE_SIMILARITY_CUTOFF: f32 = 0.6;
const OUTPUT_FILE: &str = "results.txt";

fn main() {
    let start = std::time::Instant::now();
    let stop_words: HashSet<String> = stopwords::stop_word();
    // basic method of tokenizer based on split_whitespace()
    let stopword_free_questions: Vec<HashSet<String>> = questions::questions()
        .iter()
        .map(|q| {
            q.split(" ")
                .map(|s| s.to_string())
                .filter(|s| !stop_words.contains(s))
                .collect()
        })
        .collect();

    let main_questions: Vec<HashSet<String>> = stopword_free_questions
        .iter()
        .cloned()
        .take(MAIN_LIMIT)
        .collect();

    let mut temp_vec = Vec::<(usize, Vec<usize>)>::new();
    temp_vec.reserve_exact(main_questions.len());

    main_questions
        .par_iter()
        .enumerate()
        .map(|(x_index, x_set)| {
            cosine_similarity(
                COSINE_SIMILARITY_CUTOFF,
                &stopword_free_questions,
                x_index,
                x_set,
            )
        })
        .collect_into_vec(&mut temp_vec);
    let my_map = BTreeMap::from_iter(temp_vec);
    fs::write(OUTPUT_FILE, format!("{:#?}", my_map))
        .expect(format!("Failed to write results to {}", OUTPUT_FILE).as_str());
    let end = start.elapsed();
    println!(
        "Processed similarity of {} elements in {:?}",
        my_map.len(),
        end
    );
}
