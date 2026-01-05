use std::io;
#[derive(Debug, PartialEq)]
enum Comparison {
    Equal, 
    Sublist,
    Superlist,
    Unequal,
}

fn Sublist(a: &[i32], b: &[i32]) -> Comparison{

    if a == b{
        return Comparison::Equal;
    }

    if b.is_empty() || (a.len() > b.len() && contains_sequence(a, b)) {
        return Comparison::Superlist;
    }

    if a.is_empty() || (b.len() > a.len() && contains_sequence(b, a)){
        return  Comparison::Sublist;
    }

                Comparison::Unequal
}

fn contains_sequence(bigger_list: &[i32], smaller_list: &[i32]) -> bool{
    bigger_list.windows(smaller_list.len()).any(|window| window == smaller_list)
}

fn main () {
    let mut a_vec = Vec::new();
    loop {
        println!("Insert the numbers of vector A (Insert 'end' to finalize)");
        let mut numbersa = String::new();
        io::stdin().read_line(&mut numbersa).expect("Invalid value");
        let numbersa_cleaned = numbersa.trim();

        if numbersa_cleaned== "end" {
            break;

        } else {
          
            let numbers_clean_a: i32 = numbersa.trim().parse().expect("Error");
        
            a_vec.push(numbers_clean_a);
        };
        };
   
    let mut b_vec = Vec::new();
    loop {
        println!("Insert the numbers of vector B (Insert 'end' to finalize)");
        let mut numbersb = String::new();
        io::stdin().read_line(&mut numbersb).expect("Invalid value");
        let numbersb_cleaned = numbersb.trim();

        if numbersb_cleaned== "end" {
            break;

        } else {
          
            let numbers_clean_b: i32 = numbersb.trim().parse().expect("Error");
        
            b_vec.push(numbers_clean_b);
        };
        };
   
    let results = Sublist(&a_vec, &b_vec);

    match results {
        Comparison::Equal => println!("The list A is equal to list B"),
        Comparison::Superlist => println!("The list A is a Superlist of B (A contains B)"),
        Comparison::Sublist => println!("The list A is a Sublist of B (B contains A)"),
        Comparison::Unequal => println!("They lists don't have any relation"),
    
};
}