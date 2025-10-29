use std::io;

fn main () {

    let mut vectorios = Vec::new();
    let mut summ = 0;

    println!("What is the number?");
        let mut number_str = String::new();
        io::stdin().read_line(&mut number_str).expect("Invalid value");
        
        let number_clean: i32 = number_str.trim().parse().expect("Incorrect value");
        let numberlen = number_clean.to_string().len();

    for i in 0..numberlen {

    let digit_vec = number_clean%10;

    vectorios.push(digit_vec);

    summ = summ+(vectorios[i].pow(numberlen.try_into().unwrap()));

    if summ == number_clean {
        println!("This number is armstrong");
    break;
    }

    if summ != number_clean {
        println!("This number is not a armstrong");
    break;

    }

    } 

}