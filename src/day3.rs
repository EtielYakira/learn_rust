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


    let o2 = day3_part2(data.clone(), "most_common", 0);
    //let co2 = day3_part2(data.clone(), "less_common", 0);

    //println!("o2 is: {} co2 is: {} \n The Answer is: {}", o2, co2, o2 * co2);


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



fn day3_part2(data: Vec<&str>, calc: &str, pos: usize ) -> i32 {


    let mut cntt  = 0;
    let mut data_len = data.len();
    let bit_size: usize = if let Some(f_str) = data.first() { f_str.len() } else { 0 };  
    let mut sum_of_num = vec![0; bit_size];
    let data_len_half = data_len as i32 / 2;

    for bit in  data {
        for (inx, dig) in bit.chars().enumerate() {
            if dig == '1'  {
                sum_of_num[inx] += 1
            };
        }
    }
    


    while data_len > 1 || data_len == 0 {
        println!("data len is {}, calc {}, pos {}", &data_len, calc, pos);
        let filter_data: Vec<u32> = Vec::with_capacity(data_len);
        get_filter_letter()


        cntt += 1;
    }
    
    1
}

fn get_filter_letter(data: Vec<&str>, bit_size: usize, i_to_check: i32) -> &str {
    let mut sum_of_num = 0;

    for bit in  data {
        for (inx, dig) in bit.chars().enumerate() {
            if inx == i_to_check as usize {
                if dig == '1'  {
                    sum_of_num += 1
                };
            }
        }
    }

    return "s";
}


// fn day3_part2<'a>(data: Vec<&'a str>, calc: &str, pos: usize ) -> &'a i32 {

//     //let bit_size: usize = if let Some(f_str) = data.first() { f_str.len() } else { 0 };  

//     println!("data len is {}", data.len());
//     if data.len() == 0 {
//         return &0;
//     }

//     if data.len() == 1 {
//         match i32::from_str_radix(&data[0], 2) {
//             Ok(num) => Box::leak(Box::new(num)),
//             Err(_) => &0
//         };
//     };

//     let mut sum_of_1 = 0;
//     let mut sum_of_0 = 0;

//     let hlf_data_len = data.len() as i32 / 2;

//     println!("pos : {}", pos);
//     for bit in &data {
//         let relevant_pos_char =  bit.chars().nth(pos).unwrap();
//         if relevant_pos_char == '1'  {
//             sum_of_1 += 1
//         } else if relevant_pos_char == '0' {
//             sum_of_0 += 1
//         };
//     }

//     let mut filter_data: Vec<&'a str> = Vec::new(); 
     
     

//     for bit in data {

//         println!("cacl is {} and bit is {} ", calc, , bit);
//         println!("{}", bit.chars().nth(pos).unwrap());
        
//         let mut relevant_bit = ""; 

//         match calc {
//             "most_common" =>   { 
//                 if sum_of_0 > sum_of_1 {
                    
//                 }
//                 println!("cacl is {} push bit::{}", calc, bit);
//                 filter_data.push(bit);
 
//             } ,
//             "less" => if !is_curr_bigger_than_half { 
//                 println!("cacl is {} push bit:::::::{}", calc, bit);
//                 filter_data.push(bit);

//             }  ,
//             _ => println!("{} not valid calc", calc)
//            }
//     }
//     //filter_data.push("aaa");
//     println!("filter data len {}", filter_data.len());

//     return day3_part2(filter_data, calc, pos+1);
// }

// fn day3_part2(data: Vec<&str>, calc: &str, pos: usize) -> i32 {
//     println!("data len is {}", data.len());

//     if data.is_empty() {
//         return 0;
//     }

//     if data.len() == 1 {
//         return match i32::from_str_radix(data[0], 2) {
//             Ok(num) => num,
//             Err(_) => 0,
//         };
//     }

//     let mut sum_of_pos = 0;
//     let hlf_data_len = data.len() as i32 / 2;

//     println!("pos : {}", pos);

//     for bit in &data {
//         let relevant_pos_char = bit.chars().nth(pos).unwrap();
//         if relevant_pos_char == '1' {
//             sum_of_pos += 1;
//         }
//     }

//     let mut filter_data: Vec<&str> = Vec::new();

//     for bit in data {
//         let is_curr_bigger_than_half = sum_of_pos >= hlf_data_len as i32;

//         println!("cacl is {} and is_curr_bigger_than_half is {} ", calc, is_curr_bigger_than_half);

//         match calc {
//             "greater" => {
//                 if is_curr_bigger_than_half {
//                     println!("cacl is {} push bit::{}", calc, bit);

//                     filter_data.push(bit);
//                 }
//             }
//             "less" => {
//                 if !is_curr_bigger_than_half {
//                     println!("cacl is {} push bit::{}", calc, bit);

//                     filter_data.push(bit);
//                 }
//             }
//             _ => println!("{} not valid calc", calc),
//         }
//     }

//     // Assuming you want to return the result of the recursive call
//     day3_part2(filter_data, &calc, pos + 1)
// }

