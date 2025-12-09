use itertools::Itertools;
use std::io;

fn main () {

    println!("Insert the word for possibles anagrams");
    let mut word_str = String::new();
    io::stdin().read_line(&mut word_str).expect("Invalid word");

    let word_clean = word_str.trim();

    let anagrams: Vec<String> = word_clean.chars()
    .permutations(word_clean.len())
    .unique()
    .map(|chars| chars.into_iter().collect())
    .collect();

    println!("Encountered {} unique anagrams for '{}'", anagrams.len(), word_clean);
    println!("======================================");

    for anagram in anagrams {
        println!("{}", anagram);
    }

}