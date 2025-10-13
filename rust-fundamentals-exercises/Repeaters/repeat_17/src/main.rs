use std::io;

    fn main () {

        let mut somatorie_height: f64 = 0.0;
        let mut limit = 0;
        let mut over_90: f64 = 0.0; 
        let mut weightfifthheightonesix: f64 = 0.0;
        let mut overhundred: f64 = 0.0;

    while limit<7 {
            println!("--Fale a sua altura--");
            let mut height_str = String::new();
            io::stdin().read_line(&mut height_str).expect("msg");
                let height: f64 = height_str.trim().parse().expect("msg");

            println!("--Fale o seu peso--");
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
        println!("Média de altura: [{:.2}]", somatorie_height/7.0);
        println!("Quantidade de pessoas com peso acima de 90: [{}]", over_90);
        println!("Quantidade de pessoas com peso menor que 50 kg e altura menor que 1.60m: [{}]", weightfifthheightonesix);
        println!("Quantidade de pessoas com peso maior que 100 kg e altura maior que 1.90m: [{}]", overhundred );
    }