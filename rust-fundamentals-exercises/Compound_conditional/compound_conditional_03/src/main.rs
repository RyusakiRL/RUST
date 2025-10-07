use std::io;

    const POPULAR_TERRAIN: f64 = 100.0;
    const VIP_TERRAIN: f64 = 500.0;
    fn main() {
        println!("Write the lenght of terrain in meters");
            let mut lenght_writed = String::new();
             io::stdin().read_line(&mut lenght_writed)
            .expect("Insert a valid value");

        println!("Write the height of terrain in meters");
            let mut height_writed = String::new();
            io::stdin().read_line(&mut height_writed)
            .expect("Insert a valid value");

                let lenght: f64 = lenght_writed.trim().parse().expect("msg");
                let height: f64 = height_writed.trim().parse().expect("msg");

                let area = lenght*height;
            
        if area<POPULAR_TERRAIN {
            println!("You have a Popular terrain with area {}m²", area);
        } else if 100.0<=area && area<=500.0 {
            println!("You have a Master Terrain with area {}m²", area);
        } else if area>VIP_TERRAIN {
            println!("You have a Vip Terrain with area {}m²", area);
        }

    }