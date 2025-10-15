fn main() {
    
    let mut before = 1;
    let mut before_2 = 1;
    let mut vectorios = Vec::new();

    for _ in 0..15 {

        let mut after = before + before_2;

    
        before_2 = before;
        before = after;


        vectorios.push(after);

    }

    vectorios.insert(0, 1);
    vectorios.insert(1, 1);
        for i in 0..=15 {
            
            println!("Position {} filled by {}", i, vectorios[i]);
        }

        println!("{:?}", vectorios);
}
