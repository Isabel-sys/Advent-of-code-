use std::fs::File; 
use std::io::{BufReader,BufRead};

fn main() {
    let file = File::open("day_one.input").expect("Failed to open file");

    let reader = BufReader::new(file);

    let calories : Vec<String> = reader
        .lines()
        .filter(|item|item.is_ok())
        .map(|item| item.unwrap())
        .collect();

    let split_calories: Vec<&[String]> = calories
        .split(|x|x == "")
        .collect();
    
    let mut elven_calories_raw : Vec<Vec<u32>> = Vec::new();
    
    for calories in split_calories {
        
        let nums: Vec<u32> = calories.iter().map(|item| item.parse::<u32>())
            .filter(|item| item.is_ok())
            .map(|item|item.unwrap()).collect();

        elven_calories_raw.push(nums);


    }
    let  mut elven_calories : Vec<u32> = elven_calories_raw
        .iter()
        .map(|item| item.iter().sum())
        .collect();

    elven_calories.sort();
    println!("{}",elven_calories[elven_calories.len()-1]);

    let top_three : u32 = elven_calories
        .iter()
        .rev()
        .take(3)
        .sum();

    println!("{}",top_three)
}