use std::io;

fn main () {

    println!("Insert the quantity of recite the lyric");
        let mut quantity_str = String::new();
        io::stdin().read_line(&mut quantity_str).expect("Invalid value");
        let quantity_clean: i32 = quantity_str.trim().parse().expect("Invalid value");

    println!("------BOTTLE SONG-------");

    if quantity_clean>1 {
    for i in (2..=quantity_clean).rev() {
        
        let next_count = i-1;

        let verse = format!(
        "{0} green bottles hanging on the wall,\n\
        {0} green bottles hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {1} green bottles hanging on the wall.\n",
        i, next_count
    );
    println!("{}",verse);
    }
}
    let last_verse = format!("1 green bottle hanging on the wall,\n\
                1 green bottle hanging on the wall,\n\
                And if one green bottle should accidentally fall,\n\
                There'll be no green bottles hanging on the wall.\n");

    println!("{}", last_verse);
}