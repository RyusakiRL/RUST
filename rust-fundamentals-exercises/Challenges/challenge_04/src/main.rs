
use rand::prelude::*;
use std::io;

fn main () {

       println!("Choose between rock, paper, or scissors");
       let mut po = String::new();
       io::stdin().read_line(&mut po).expect("Insert a valid value");
       
        let poke: String = po.trim().to_lowercase().parse().expect("msg");
       
        let mut random = rand::rng();
        let x = random.random_range(1..=3);

    let joken: String = match x {
        1 => "Rock".to_string(),
        2 => "Paper".to_string(),
        3 => "Scissor".to_string(),
        _ => "Error: Inespered value".to_string(),
    };
   
        if poke == "rock" && x==1 {
            println!("The machine choose {}, draw", joken);
        } else if poke == "rock" && x==2 {
            println!("The machine choose {}, you lost", joken);
        } else if poke == "rock" && x==3 {
            println!("The machine choose {}, you win", joken);
        }

        if poke == "paper" && x==1{
            println!("The machine choose {}, you win", joken);
        } else if poke == "paper" && x==2 {
            println!("The machine choose {}, draw", joken);
        } else if poke == "paper" && x==3 {
            println!("The machine choose {}, you lost", joken);
        }

        if poke == "scissor" && x==1 {
            println!("The machine choose {}, you lost", joken);
        } else if poke == "scissor" && x==2 {
            println!("The machine choose {}, you win", joken);
        } else if poke == "scissor" && x==3 {
            println!("The machine choose {}, draw", joken);
        }
}