fn sanji (message:&str, vezes: i32) {
   
    let visual_line = "+-------=======------+";
        println!("{}", visual_line);
    for _ in 0..vezes {
        println!("{}", message);
    }
        println!("{}", visual_line)
}

fn main () {

    sanji("Learning Rust", 4);
}

