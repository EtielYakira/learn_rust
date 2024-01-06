use core::num;
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



    println!("Gamma is: {} Epsilon is: {} \n The Answer is: {}", gamma, epsilon, gamma * epsilon);


    let o2 = day3_part2(data.clone(), "most_common");
    let co2 = day3_part2(data.clone(), "less_common");

    println!("o2 is: {} co2 is: {} \n The Answer is: {}", o2, co2, o2 * co2);


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

/*
task : ans = o2 * co2
find commen(o2)/uncomment(co2) in the curr bit pos
if equel keep values with 1(o2)/0(co2)
filter out all bits that not have

if leaft with 1 save him

*/


fn day3_part2(data: Vec<&str>, calc: &str) -> i32 {
    let bit_size: usize = if let Some(f_str) = data.first() { f_str.len() } else { 0 };  
    println!("{:?}", data);

    let mut new_data: Vec<&str> = data.clone();  // Change the type to Vec<&str>
    let mut curr_pos = 0;

    while new_data.len() > 1 && curr_pos < bit_size as i32 {

        let sum_of_curr_bit = new_data
            .iter()
            .fold(0, |acc, x| acc + x.chars().nth(curr_pos as usize).unwrap() as i32 - 0x30);
        println!("{:?}", sum_of_curr_bit);
        let temp = (new_data.len() as f32 / 2.0) as f32;
        let data_len_half = temp.ceil() as i32;
        println!("{:?} data_len_half", data_len_half);

        let number_to_check = match calc {
            "most_common" => if sum_of_curr_bit == data_len_half {1} else if sum_of_curr_bit > data_len_half { 1 } else { 0 },
            "less_common" => if sum_of_curr_bit  == data_len_half {0} else if sum_of_curr_bit > data_len_half { 0 } else { 1 },
            _ => 2
        };
        println!("{:?}", number_to_check);

        let mut filtered_data = Vec::new();
        for &x in &new_data {
            println!("{:?}", x.chars().nth(curr_pos as usize).unwrap().to_digit(10).map(|digit| digit as i32).unwrap());
            if x.chars().nth(curr_pos as usize).unwrap().to_digit(10).map(|digit| digit as i32).unwrap() == number_to_check {
                filtered_data.push(x);
            }
        }
        new_data = filtered_data;
        println!("{:?}", new_data);
        curr_pos += 1;
    }

    return match i32::from_str_radix(new_data.first().unwrap(), 2) {
        Ok(num) => num,
        Err(_) => 0
    }; 
}
