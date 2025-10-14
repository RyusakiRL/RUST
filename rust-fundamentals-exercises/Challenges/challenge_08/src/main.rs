fn main() {
 
    let mut before: i32 = 1;
    let mut before_2: i32 = 1;
    
    print!("{} {} ", before_2, before); 


    for _ in 0..8 { 
        
  
        let after = before + before_2;
        
    
        print!("{} ", after);

        before_2 = before;
        
        before = after;
    }
    
    println!("END!!");
}