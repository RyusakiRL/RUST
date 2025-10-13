use std::io;

fn main () {

    let mut somatorie = 0;

loop {

    println!("---Insert number---");
    let mut number_str = String::new();
    io::stdin().read_line(&mut number_str).expect("msg");
    
        let number: i32 = number_str.trim().parse().expect("msg");

            somatorie = somatorie + number;
    if number == 1111 {
        break;
    }
}
    println!("The somatorie is [{}]", somatorie - 1111);
}