fn main() {

    {

        let s = -48i32;
        let s2 = s.abs();
    }


    let own_num = 5;
    let borrow_num: &i32 = &own_num;

    println!("{}", borrow_num);

    let borrow_num2 = &own_num;
    my_funciton(borrow_num2);
    println!("{}", borrow_num2);

    

}

fn my_funciton(num: &i32){
    
    println!("{}", num);
}
