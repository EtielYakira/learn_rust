use std::fs;
use std::path::Path;
use std::collections::HashMap;


pub fn day2(){
    let file_path = Path::new("./DATA/day2.txt");
    let content = fs::read_to_string(file_path)
        .expect("need to read file");

    let mut actions: HashMap<&str, Vec<i32>> = HashMap::new();

    for act in content.lines() 
    {
        let parts: Vec<&str> = act.split(' ').collect();

        actions.entry(parts[0])
        .or_insert_with(Vec::<i32>::new)
        .push(parts[1].parse::<i32>().unwrap());

    }
    
    let ans = day2_task1(actions);
    println!("{ans}")
}

fn day2_task1(actions: HashMap<&str, Vec<i32>>) -> i32
{
    let mut depth  = 0;
    let mut horizontal = 0;

    for (action, numbers) in actions {
        match action {
            "forward" => horizontal += numbers.iter().sum::<i32>(),
            "up" => depth -= numbers.iter().sum::<i32>(),
            "down" => depth += numbers.iter().sum::<i32>(),
            _ => {}
        }
    }

    return depth * horizontal;
}