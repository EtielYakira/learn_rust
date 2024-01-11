pub fn day4(){
    let file_path = "";
    let file_as_string = read_file_to_str(&file_path);
        

    let (boards, winning_numbers) = create_game_from_string(file_as_string);
    
    




}

fn create_game_from_string(file_as_string: &str) -> (Vec<i32>,) {
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