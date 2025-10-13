use std::io;

    fn main () {

        let mut somatorie_height: f64 = 0.0;
        let mut limit = 0;
        let mut over_90: f64 = 0.0; 
        let mut weightfifthheightonesix: f64 = 0.0;
        let mut overhundred: f64 = 0.0;

    while limit<7 {
            println!("--Height--");
            let mut height_str = String::new();
            io::stdin().read_line(&mut height_str).expect("msg");
                let height: f64 = height_str.trim().parse().expect("msg");

            println!("--Weight--");
            let mut weight_str = String::new();
            io::stdin().read_line(&mut weight_str).expect("msg");
                let weight: f64 = weight_str.trim().parse().expect("msg");

        somatorie_height = somatorie_height + height;
        limit = limit + 1;

            if weight>90.0 {
                  over_90 = over_90 + 1.0;
            }

            if weight<50.0 && height<1.6 {
                weightfifthheightonesix = weightfifthheightonesix + 1.0;
            }

            if weight>100.0 && height>1.9 {
                overhundred = overhundred + 1.0;
            }
    }
        println!("Average of height: [{:.2}]", somatorie_height/7.0);
        println!("Quantity of people with weight over 90 kg: [{}]", over_90);
        println!("Number of people weighing less than 50 kg and less than 1.60 m tall: [{}]", weightfifthheightonesix);
        println!("Number of people weighing more than 100 kg and taller than 1.90 m: [{}]", overhundred );
    }