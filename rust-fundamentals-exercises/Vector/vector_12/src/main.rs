use std::io;

fn main (){

    let mut vectorios = Vec::new();
    let mut summ_score = 0.0;
    let mut bigger_score = 0.0;
    let mut overaverage = 0;
    let mut position = 0;

    for _ in 1..=9 {
        
        println!("--Insert the exam score--");
            let mut score_str = String::new();
            io::stdin().read_line(&mut score_str).expect("Invalid number");
                let score: f64 = score_str.trim().parse().expect("Inespered error");

        vectorios.push(score);
        summ_score+=score;


    }
    
    let average = summ_score/10.0;
        
    println!("--Results");
    println!("The average score of class: [{}]", average);
    
    for i in 0..vectorios.len(){

    if vectorios[i]>average {
        overaverage = overaverage + 1;
    }

    if vectorios[i]>bigger_score {
        bigger_score = vectorios[i];
        position = i
    }

    }
    println!("How many people have age over average: [{}]", overaverage);
    println!("The bigger score is: [{}]", bigger_score);
    println!("Encountered in position: [{}]", position);
}