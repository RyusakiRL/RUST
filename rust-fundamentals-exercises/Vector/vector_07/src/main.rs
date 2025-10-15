use std::io;

fn main (){

    let mut vectorios = Vec::new();

    for _ in 0..=6 {
        
        println!("--Insert the Name--");
            let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Inespered error");
                    let name_clean: String = name.trim().to_string();

        vectorios.push(name_clean);
    
    }

    for i in (0..=6).rev() {
         println!("Position {} filled by {}", i, vectorios[i]);
    }

        println!("Final vector ({} positions)", vectorios.len());
        println!("{:?}", vectorios);

}