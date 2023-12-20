use std::{path::Path, fs};



pub fn day3() {
    let file_path = Path::new("./DATA/day3.txt");
    let content = fs::read_to_string(file_path)
        .expect("need to read file");

}