use std::io;

    fn main (){
        println!("What is your name?");
        
            let mut name = String::new();
            io::stdin().read_line(&mut name)
            .expect("msg");

        println!("Write the first note");

            let mut first = String::new();
            io::stdin().read_line(&mut first)
            .expect("msg");

        println!("Write the second note");

            let mut second = String::new();
            io::stdin().read_line(&mut second)
            .expect("msg");

                let first_note : f64 = first.trim().parse().expect("msg");
                let second_note: f64 = second.trim().parse().expect("msg");

            let media: f64 = (first_note+second_note)/2.0;

                println!("Hello {}, your first note is {} and second is {}, the media is {}", name.trim(), first_note, second_note, media);

                        if media>7.0 {
                                
                                println!("Congratulations, you have a great perfomance");
                            
                        } else {
                                println!("your performance is bad, I recommend studying more");
                        }

    }