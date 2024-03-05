use weighted_rand::builder::*;
use std::collections::HashMap;
use std::fs;
use rand::seq::SliceRandom; 
use std::io;
fn main() {
    let file_path: &str = "dataset.txt";
    //User input 
    println!("Dataset from file: {}", file_path);
    println!("Enter how many words to store in each node: ");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to load line");
    let n_gram: usize = input_line.trim().parse().expect("Input not an integer");
    println!("Enter how many nodes you want: ");
    input_line.clear();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to load line");
    let mut num_words: u32 = input_line.trim().parse().expect("Input not an integer");
    num_words = num_words - 1; 

    println!("Model working... please wait");
    let contents = fs::read_to_string(file_path).expect("Failed reading file");
    let words: Vec<&str> = contents.split(";").collect();
    let model = make_markov_model(words, n_gram);
    let story: String = generate_story(model.clone(), num_words);
    println!("{story}");
}

fn make_markov_model(dataset: Vec<&str>, n_gram: usize) -> HashMap<String, HashMap<String, f32>> {
    let mut markov_model: HashMap<String, HashMap<String, f32>> = HashMap::new();
    //for i in 0..(dataset.len() - n_gram - 1) {
    for i in 0..(dataset.len() - 2*n_gram + 1) {
        // i don't see how that works for n_gram other
        // than 2 and 1
        let mut curr_state: String = "".to_string();
        let mut next_state: String = "".to_string();
        for j in 0..n_gram {
            curr_state.push_str(dataset[i + j]);
            curr_state.push_str(" ");
            next_state.push_str(dataset[i + j + n_gram]);
            next_state.push_str(" ");
        }
        //delete the last char, since it is going to be space
        curr_state.pop();
        next_state.pop();

        // if not curr_state in model, create it
        if !markov_model.contains_key(&curr_state) {
            markov_model.insert(curr_state, HashMap::from([(next_state, 1.0)]));
        }
        //if curr_state in model, check if next_state in model
        else if markov_model[&curr_state].contains_key(&next_state) {
            // if next state in model, then we increment
            markov_model.entry(curr_state).and_modify(|x| {
                x.entry(next_state).and_modify(|y| {
                    *y += 1.0;
                });
            });
        }
        // if curr_state in model, next_state not in model, create next_state
        else {
            markov_model.entry(curr_state).and_modify(|x| {
                x.insert(next_state, 1.0);
            });
        }
    }
    // calculate sum
    // change every value to probablity
    for value in markov_model.values_mut() {
        let mut sum: f32 = 0.0;
        for internal_val in value.values_mut() {
            sum = sum + *internal_val;
        }
        for internal_val in value.values_mut() {
            *internal_val = *internal_val / sum;
        }
    }
    return markov_model;
}

fn generate_story(model: HashMap<String, HashMap<String, f32>>, limit: u32) -> String {
    let mut result: String = "".to_string();
    let mut n: u32 = 0;
    let keys: Vec::<String> = model.clone().into_keys().collect();
    let key: String = keys.choose(&mut rand::thread_rng()).unwrap().to_string();
    let start: String = key;
    let mut curr_state: String = start; 
    result.push_str(&curr_state);
    result.push(' ');
    // create a list from model[curr_state], and use weights to randomly choose next_state
    while n < limit {
        let possible: Vec<(String, f32)> = model[&curr_state].clone().into_iter().collect();
        let mut weights: Vec<f32> = vec![];
        for i in 0..possible.len() {
            weights.push(possible[i].1);
        }
        let mut words: Vec<String> = vec![];
        for i in 0..possible.len() {
            words.push(possible[i].0.clone());
        }
        let builder = WalkerTableBuilder::new(&weights);
        let wa_table = builder.build();
        let next_state: String = possible[wa_table.next()].0.clone();
        curr_state = next_state;
        result.push_str(&curr_state);
        result.push(' ');
        n = n + 1;
    }
    return result;
}
