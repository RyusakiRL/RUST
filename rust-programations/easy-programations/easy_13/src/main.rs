use std::collections::HashSet;
use std::io;

fn energy_points(level: u32, items: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for &base_value in items {
        let mut multiple = base_value;
        
        while multiple < level {
            
            multiples.insert(multiple);
            multiple += base_value;
        }
    }

        multiples.iter().sum()

}


fn main () {

        println!("---Calculator of energy points---");
        
        println!("Insert the completed level");
        let mut level_str = String::new();
        io::stdin().read_line(&mut level_str).expect("Invalid value");
        let level: u32 = level_str.trim().parse().expect("Error");

        println!("Enter the value of magical items (Splitted with space)");
        let mut items_str = String::new();
        io::stdin().read_line(&mut items_str).expect("Invalid values");

        let items: Vec<u32> = items_str
        .split_whitespace()
        .map(|s|s.parse::<u32>().expect("Invalid value"))
        .collect();

        let point_of_energy = energy_points(level, &items);
        println!("The player won {} energy points!!", point_of_energy);
        




}