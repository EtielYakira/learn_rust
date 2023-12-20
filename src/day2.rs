use std::fs;
use std::path::Path;
use std::collections::HashMap;


pub fn day2(){
    let file_path = Path::new("./DATA/day2.txt");
    let content = fs::read_to_string(file_path)
        .expect("need to read file");



    let mut actions: HashMap<&str, Vec<i32>> = HashMap::new();
    let lines: Vec<(&str, i32)> = content
    .lines()
    .enumerate()
    .map(|(_, x)|  {
        let  parts: Vec<&str> = x.split(' ').collect();

        return (parts[0], parts[1].parse::<i32>().unwrap());
    })
    .collect();

    for act in content.lines() 
    {
        let parts: Vec<&str> = act.split(' ').collect();

        actions.entry(parts[0])
        .or_insert_with(Vec::<i32>::new)
        .push(parts[1].parse::<i32>().unwrap());

    }
    
    let ans = day2_task1(actions);
    let ans2 = day2_task2(lines);
    println!("{ans}");
    println!("{ans2}");
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

fn day2_task2(lines: Vec<(&str, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut depth  = 0;
    let mut aim  = 0;

    for action in lines {
        let act = action.0;
        let num = action.1;

        match act {
            "forward" => {
                horizontal += num;
                depth += aim * num;
            } ,
            "up" => aim -= num ,
            "down" => aim += num,
            _ => {}
        }

    }

    return depth * horizontal;
}