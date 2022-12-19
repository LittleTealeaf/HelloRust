use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Green"),50);


    println!("Get scores! {}",scores.get(&String::from("Blue")).expect(""));
}
