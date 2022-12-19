use std::collections::HashMap;
use std::io;

fn main() {
    println!("Please type a sentence");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let words = to_words(&sentence);
    let piglatin = convert_pig_latin(words);
    println!("{}",piglatin.join(" "));
}

fn to_words(string: &str) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    let mut word_start = 0;

    for (i, &byte) in string.as_bytes().iter().enumerate() {
        if byte == b' ' {
            words.push(&string[word_start..i]);
            word_start = i + 1;
        }
    }

    words.push(&string[word_start..]);

    words
}

fn convert_pig_latin(words: Vec<&str>) -> Vec<String> {
    let mut pig_latin: Vec<String> = Vec::new();

    let mut vowels = HashMap::new();
    let vowels_str = String::from("aeiouAEIOU");
    for (_, vowel) in vowels_str.as_bytes().iter().enumerate() {
        vowels.insert(vowel, 1);
    }

    for (_, word) in words.iter().enumerate() {
        if let Some(first) = word.as_bytes().first() {
            if let Some(_) = vowels.get(first) {
               pig_latin.push({
                   let mut my_word = word.to_string();
                   my_word.push_str("-hay");
                   my_word
               }); 
            } else {
                pig_latin.push({
                    let mut my_word = word[1..].to_string();
                    my_word.push_str("-");
                    my_word.push_str(&word[..1]);
                    my_word.push_str("ay");
                    my_word
                })
            }
        }
        //if let Some(_) = vowels.get(word.as_bytes().first()) {}
    }

    pig_latin
}
