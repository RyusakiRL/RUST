
fn main () {

    let mut vectorio = Vec::new();

    for i in 0..=7 {
        vectorio.push(999);

        println!("Position {} filled with: {}", i, vectorio[i] );
    }

    println!("Final Vector ({} positions)", vectorio.len());
    println!("{:?}", vectorio);


}