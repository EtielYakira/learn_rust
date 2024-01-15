use std::{path::Path, fs};
use itertools::Itertools;


pub fn day4(){
    let file_path = Path::new("./src/DATA/day4.txt");
    let content = fs::read_to_string(file_path)
        .expect("need to read file");




    let (boards, winning_numbers) = create_game_from_string(content);
    
    




}

fn create_game_from_string(file_as_string: String) -> (Vec<i32>,Vec<i32>) {
    
    let data: Vec<&str> = file_as_string 
    .lines()
    .map(|x| x)
    .collect();


    let line_winning_numbers: Vec<i32> = data[0].split(',').map(|s| s.parse().unwrap()).collect();

    let games_str = &data[1..];    

    let games = games_str.iter().filter(|x| x.is_empty()).chunks(5);
    todo!()
}


fn read_file_to_str(file_path: &str) -> &str {
    todo!()
}

pub trait Game {
    
}

pub struct Bingo {
    pub table: Table 
}

pub struct Table{
    pub rows: Vec<i32>,
    pub colums: Vec<i32>
}