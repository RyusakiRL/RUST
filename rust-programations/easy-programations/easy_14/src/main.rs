use std::io;

fn bob (text: String) -> String{

    if text == text.to_ascii_uppercase() {
        println!("Whoaa, chill out!")
    };
    if text.contains('?') && text!= text.to_ascii_uppercase(){
        println!("Sure.")
    };

    if text.contains('?') && text == text.to_ascii_uppercase() {
        println!("Calm down, i know what i'm doing");
    };

    if text.is_empty() {
        println!("Fine. Be that way!")
    };

    if !text.is_empty() && text.contains('?') && text!= text.to_ascii_uppercase() {
        println!("Whatever")
    };

    "bob answer".to_string()
}  

    

fn main() {

    println!("---Insert the text for Bob---");
    let mut text_str = String::new();
    io::stdin().read_line(&mut text_str).expect("Invalid text");
    
    bob(text_str);

}