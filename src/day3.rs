use std::{path::Path, fs};



pub fn day3() {
    let file_path = Path::new("./DATA/day3.txt");
    let content = fs::read_to_string(file_path)
        .expect("need to read file");

    let data: Vec<&str> = content
    .lines()
    .map(|x| x)
    .collect();
    


    let epsilon  = day3_part1(data.clone(), "grather");
    let gamma  = day3_part1(data.clone(), "less");

    println!("Gamma is: {} Epsilon is: {} \n The Answer is: {}", gamma, epsilon, gamma * epsilon)

}


// for gamma (epsilon same but if less then one else 0) 
//carete fix ssize array lent of line * bits
//  iter over list and and add the number
// if grether then hald sszie of lines 1 ekse 0
//  build the bit

fn day3_part1(data: Vec<&str>, calc:&str ) -> i32{

    let bit_size: usize = if let Some(f_str) = data.first() { f_str.len() } else { 0 };  

    let mut sum_of_num = vec![0; bit_size];


    let data_len = data.len() as i32 / 2;

    for bit in  data {
        for (inx, dig) in bit.chars().enumerate() {
            if dig == '1'  {
                sum_of_num[inx] += 1
            };
        }
    }

    let mut relevant_bit: String = "".to_owned();

    for num in sum_of_num {
        let is_curr_bigger_than_half = num > data_len as i32;
        match calc {
            "grather" => if is_curr_bigger_than_half  { relevant_bit.push_str("1") } else {  relevant_bit.push_str("0") },
            "less" => if is_curr_bigger_than_half  { relevant_bit.push_str("0") } else {  relevant_bit.push_str("1") },
            _ => println!("{} not valid calc", calc)
           }
    }
    
    print!("RS:{} CACL:{}", relevant_bit, calc);

    return match i32::from_str_radix(&relevant_bit, 2) {
        Ok(num) => num,
        Err(_) => 0
    }
}

