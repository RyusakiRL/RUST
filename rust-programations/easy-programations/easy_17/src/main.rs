use std::io;

fn series (digits: &str, len: usize)-> Vec<String>{

    let input_len = digits.len();

    if len > digits.len() && len == 0 {
        return Vec::new();
    }

    (0..=(input_len - len)).map(|i| {

        digits[i..(i + len)].to_string()
        
    }).collect()
}

fn main () {

    println!("---Insert your number---");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Invalid value");

    let len_3 = 3;
    let len_4 = 4;
    let len_6 = 6;


    println!("---Results---");
    println!("Series of {}: {:?}", len_3, series(number.trim(), len_3) );
    println!("Serie of {}: {:?}", len_4, series(number.trim(), len_4));
    println!("Series of {}: {:?}", len_6, series(number.trim(), len_6));
}