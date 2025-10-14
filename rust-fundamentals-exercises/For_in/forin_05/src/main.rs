use std::io;

fn main () {

    let mut quantitywoman = 0.0;
    let mut quantityman = 0.0;
    let mut summ =0.0;
    let mut highestweightman = 0.0;

for _n in 0..=8 {
    
    println!("---Insert the gender Woman or Man---");
        let mut gender_str = String::new();
        io::stdin().read_line(&mut gender_str).expect("Insert woman or man");
            let gender = gender_str.trim().to_lowercase();
    
    println!("---Insert the height---");
        let mut weight_str = String::new();
        io::stdin().read_line(&mut weight_str).expect("Insert a number");
            let weight: f64 = weight_str.trim().parse().expect("It's not a possible value");
    

    if gender == "woman" {
        quantitywoman += 1.0;
        summ += weight
    }
    
    if weight>100.0 && gender == "man" {
        quantityman += 1.0;
    }

    if weight>highestweightman {
        highestweightman = weight;
    }

}
        println!("---Results---");
        println!("How many womans have been registered: [{}]", quantitywoman);
        println!("How many mans have weight over hundred: [{}]", quantityman);
        println!("Average weight of womans: {:.2} kg", summ/quantitywoman);
        println!("The bighest weight of mans: {:.2} kg", highestweightman );


}