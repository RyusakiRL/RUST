use std::io;

fn average (first: f64, second: f64) ->f64 {
    (first+second)/2.0
}

fn main () {
    println!("--Insert the first score--");
        let mut firstscore_str = String::new();
        io::stdin().read_line(&mut firstscore_str).expect("Invalid number");
            let firstscore: f64 = firstscore_str.trim().parse().expect("Invalid number");

    println!("--Insert the second score--");
        let mut secondscore_str = String::new();
        io::stdin().read_line(&mut secondscore_str).expect("Invalid number");
            let secondscore: f64 = secondscore_str.trim().parse().expect("Invalid number");

    let average = average(firstscore, secondscore);

    println!("The average is {}", average);

    if average>6.0 {
        println!("APROVED");
    }

    if average<3.0 {
        println!("REPROVED");
    }

    if average>3.0 && average<6.0 {
        println!("EXAM");
    }
}