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

    println!("{}", my_funciton(33, "thrid_thid".to_string()));
    //println!("{}",tup);    
    my_funciton2();
    my_loop();
}



fn my_funciton(x: i32, name: String) -> i32{
    println!("this {}, with the name of {}", x, name);
    x
}


fn my_funciton2() {
    let number = 5;
     
     if number > 10 {
        println!("first")
     }
     else if number > 22 {
         println!("sec")
     }
     else {
         println!("last")
     }

     let cond = 1 == 1;
     let number = if cond { 5 } else { 6 };

     println!("{}", number);
}



fn my_loop() {
    loop {
        print!("hi");
        break;
    };

    let mut cnt = 0;
    while cnt <= 10 {
        println!("{0} is the number {1}", cnt , cnt + 1 );
        cnt += 1;
    };

    let arr = [1,2,3,4,5,6];
    for item in arr {
        
        println!("{} number int arr", item);
    };
    println!("{:?}", &arr[1..5]);
    let annotheraar = [2,3,4,5];
    let mut copy_arr = [0; 4];
    copy_arr.clone_from_slice(&arr[1..5]);
    println!("{:?}", copy_arr);
    //assert_eq!(copy_arr.len() +1, annotheraar.len(), " test of {} and {} " ,copy_arr.len(), annotheraar.len());
    // print!(" {}", assert_eq!(copy_arr.len(), annotheraar.len()));
    

    for numt in 1..3{
        println!("{}", numt);
    };

}