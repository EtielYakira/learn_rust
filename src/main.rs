fn main() {
    let mut x  = 5;
    println!("val is  {}", x);
    x = 4;

    println!("vale is {}", x);
    
    const DREAM_RUST: u32 = 10_000;
    
    println!("vale is {}", DREAM_RUST);


    let tup = ("hi", 33, 44);
    let (word, _num, _) = tup;
    let _wordy = tup.1;

    println!("{:?}",tup);    
    println!("{}",word);    
    //println!("{}",tup);    

}
