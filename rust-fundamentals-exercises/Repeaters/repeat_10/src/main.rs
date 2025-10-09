fn main () {
    
    let mut number = 500;
    let mut soma = 0;

    while number!=0 {
        println!("[{}]", number);
        number = number - 50;
        soma = soma + number;
        
    }
        println!("The value of some is {}", soma);
}