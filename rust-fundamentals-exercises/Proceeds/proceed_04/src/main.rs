use std::io;

fn superman (message: &str, times: i32, choosing: i32) {

    let visual_lineone = "+-------=======------+";
    let visual_linetwo = "~~~~~~~~:::::::~~~~~~~";
    let visual_linethree = "<<<<<<<<------->>>>>>>";

    if choosing == 1 {
        println!("{}", visual_lineone);
    }

    if choosing == 2 {
        println!("{}", visual_linetwo);
    }

    if choosing == 3 {
        println!("{}", visual_linethree);
    }

    for _ in 1..times {
        println!("{}", message);    
    }

        if choosing == 1 {
        println!("{}", visual_lineone);
    }

    if choosing == 2 {
        println!("{}", visual_linetwo);
    }

    if choosing == 3 {
        println!("{}", visual_linethree);
    }

}

fn main () {

    println!("What your prefer? (1, 2, 3)");
    println!("Border one: [+-------=======------+]");
    println!("Border two: [~~~~~~~~:::::::~~~~~~~]");
    println!("Border three: [<<<<<<<<------->>>>>>>]");
        let mut choose_str = String::new();
        io::stdin().read_line(&mut choose_str).expect("Invalid number");
            let choose: i32 = choose_str.trim().parse().expect("Invalid number");
        
    superman("Learning Rust.", 4, choose);


}