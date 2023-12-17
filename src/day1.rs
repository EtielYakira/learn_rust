// use std::env;
use std::fs;
use std::path::Path;


pub fn day1(){
    let file_path = Path::new("./DATA/day1.txt");
    let content = fs::read_to_string(file_path)
        .expect("need to read file");

    let list_of_numbers_str: Vec<i32> = content
        .lines()
        .flat_map(|x| x.parse().ok())
        .collect();
    
    let ans = get_large_measurements_part_two(list_of_numbers_str);


    println!("{ans}");

}


fn _get_large_measurements(measurements: Vec<i32>) -> i32 {
    return measurements
    .iter()
    .enumerate()
    .fold(0, |acc, (i, _x )| {
        println!("{i}");
        println!("{} the innum", measurements[i]);
        println!("{acc} this is aac");
        //println!("{} the innum-1", measurements[i-1]);
        if i > 0 {
            if measurements[i] > measurements[i-1]{
                return acc + 1;
            }
        }
        return acc;
    });
}



fn get_large_measurements_part_two(measurements: Vec<i32>) -> i32 {
    return measurements
    .iter()
    .enumerate()
    .fold(0, |acc, (i, _x )| {
        
        if i + 3 >= measurements.len() {
            return acc;
        }

        if measurements[i..i+3].iter().sum::<i32>() < measurements[i+1..i+4].iter().sum() {
            return acc + 1;
        }

        // let mut curr = 0; 
        // let mut prev = 0;
        
        // let tetst = measurements[i..i+3];
        // let tetstt = measurements[i+3..i+6];



        // if i > 0 && i < 3
        // {
        //     curr = measurements[0..i].iter().sum();
        //     prev = measurements[0..i-1].iter().sum();
        //     // if  > measurements[i-1]{
        //     //     return acc + 1;
        //     // }
        // }

        // if i > 3  
        // {
        //     curr = measurements[0..i].iter().sum();
        //     prev = measurements[0..i-1].iter().sum();
        //     // measurements[0..i-1].iter().sum() < measurements[0..i].iter().sum();
        // }

        // if curr > prev 
        // {
        //    return acc + 1;
        // }


        // let curr_nums = [0, 3];
        // let prev_num = [0,3];
        // let mesurment_size = measurements.len();
        // if i < 3 {
        //     if i > 0 {
        //         if measurements[i] > measurements[i-1]{
        //             return acc + 1;
        //         }
        //     }
        // }
        return acc;
    });
}
