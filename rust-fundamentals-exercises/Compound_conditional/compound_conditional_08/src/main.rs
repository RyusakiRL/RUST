use std::io;

    const TENHOURS_ACTIVITY: f64 = 2.0;
    const VALUE_POINT: f64 = 0.05;

fn main () {
        println!("Welcome to healthy life program, how many hours of activities you do in this month?");
        let mut hours_writed = String::new();
        io::stdin().read_line(&mut hours_writed).expect("msg");

            let hour: f64 = hours_writed.trim().parse().expect("msg");
        if hour<=10.0 {
            println!("You exercited {} hours in this month and gained U${}", hour, TENHOURS_ACTIVITY*hour*VALUE_POINT);

        } else if 10.0<hour && hour<=20.0  {
            println!("You exercited {} hours in this month and gained U${}", hour, (20.0+(hour-10.0)*5.0)*VALUE_POINT);

        } else if hour>20.0 {
            println!("You exercited {} hours in this month and gained U${}",hour, (70.0+(hour-20.0)*10.0)*VALUE_POINT);
        }

}