use std::io;

fn main () {
   
    let mut max = 0;

    println!("--Welcome insert the points to register (Splitted with space)--");
    let mut points_str = String::new();
    io::stdin().read_line(&mut points_str).expect("Invalid value");
    
    let scores: Vec<i32> = points_str.split_whitespace()
    .map(|s|s.parse::<i32>().expect("invalid value"))
    .collect();

    for i in 0..scores.len(){
        
        if scores[i]>max {
          max = scores[i];  
        }
    }
    println!("----RESULTS----");
    println!("The highest score is {}", max);
}