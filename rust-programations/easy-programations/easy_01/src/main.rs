use unicode_segmentation::UnicodeSegmentation;
use std::io;

fn reverse (s: &str) -> String{

    s.graphemes(true).rev().collect()

}

fn main () {

    println!("--Insert the word--");
        let mut word_str = String::new();
        io::stdin().read_line(&mut word_str).expect("Invalid string");
            let word_clean = word_str.trim().to_string();

    let reversal = reverse(&word_clean);

    println!("The word original is: {}", word_clean);
    println!("In reversal format is: {}", reversal);
}