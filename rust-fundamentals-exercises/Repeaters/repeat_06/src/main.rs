

    fn main() {

        let mut number = 30;

        while number!=1 {
           
            number = number - 1;
            
            if number%4 == 0 {
                println!("[{}]", number);
            } else {
                println!("{}", number);
            }

        }
                println!("End!!");
    }