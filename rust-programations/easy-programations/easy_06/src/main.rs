fn main () {
    
        let mut chessboard = 1;
        let mut summ:i128 = 0;
    
    println!("------RESULTS------");
    println!("Quantity of grains in square position 1 is 1");

    for i in 2..=64 {
        chessboard = chessboard*2;
        summ = summ+ chessboard;

        println!("Quantity of grains in square position {} is {}", i, chessboard);
    }
    
    println!("The total of grains on the chessboard is {}", summ);


}