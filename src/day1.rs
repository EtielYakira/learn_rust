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
    
    let ans = get_large_measurements(list_of_numbers_str);


    println!("{ans}")

}


fn get_large_measurements(measurements: Vec<i32>) -> i32 {
    // measurements.
    return 0;
}
