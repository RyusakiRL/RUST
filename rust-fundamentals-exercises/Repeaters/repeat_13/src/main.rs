use rand::prelude::*;

    fn main () {

        let mut rng = rand::rng();
        let mut bigfive = 0;
        let mut divisiblethree = 0;
        let mut limit = 0;
        
        while limit<20 {
            limit=limit + 1;
              let number= rng.random_range(1..=10);
                println!("[{}]", number);
        if number>5 {
            bigfive = bigfive + 1;
        } else if number%3 == 0 && number!=0 {
            divisiblethree = divisiblethree + 1;
        }
   }

        println!(" ---Results---");
        println!("Number above five [{}]", bigfive);
        println!("Numbers divible per three [{}]", divisiblethree);
    }