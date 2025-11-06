fn generate_proverb(parts: &[&str]) -> String {
    if parts.is_empty() {
        return String::new(); 
    }

    let mut output = String::new();
    let n = parts.len();


    for i in 0..(n - 1) {
        let first_word = parts[i];
        let second_word = parts[i + 1];
        

        let line = format!(
            "For want of a {} the {} was lost.\n",
            first_word,
            second_word
        );
        output.push_str(&line);
    }


    let first_word = parts[0];
    let closing_line = format!("And all for the want of a {}.", first_word);
    output.push_str(&closing_line);

    output
}

fn main() {
    // Exemplo de entrada (como no problema)
    let input_list = vec!["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];

    let proverb = generate_proverb(&input_list);
    
    println!("{}", proverb);

    println!("\n--- Other list ---");
    let short_list = vec!["Happy", "Sad", "Angry", "Patience", "Lust", "Chastity"];
    println!("{}", generate_proverb(&short_list));
}