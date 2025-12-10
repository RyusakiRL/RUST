use std::io;

fn main () {



    let planets = vec![
        ("mercury", 0.2408467),
        ("venus", 0.61519726),
        ("earth", 1.0),
        ("mars", 1.8808158),
        ("jupiter", 11.862615),
        ("saturn", 29.447498),
        ("uranus", 84.016846),
        ("neptune", 164.79132),
    ];

    println!("What is the planet your entered?");
    let mut planet_str = String::new();
    io::stdin().read_line(&mut planet_str).expect("Invalid value");
    let planet_clean: String = planet_str.trim().to_ascii_lowercase();

    println!("How many years have you lived on this planet?");
    let mut years_str = String::new();
    io::stdin().read_line(&mut years_str).expect("invalid value");
    let years: f64 = years_str.trim().parse().expect("Invalid value");

        let mut actual_age_in_earth: f64 = 0.0;

    for i in 0..7 {
      if planet_clean == planets[i].0 {
          
        actual_age_in_earth = years*planets[i].1;
      }
    }
    println!("-----------RESULTS-----------");
    println!("Your actual space age in seconds is {}", actual_age_in_earth*31557600.0);


}