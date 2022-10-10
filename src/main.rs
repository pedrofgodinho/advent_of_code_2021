use std::collections::HashMap;
use std::fs;

use advent_of_code_2021::problems;
use advent_of_code_2021::problems::Problem;

fn main() {
    let mut problems: HashMap<usize, Box<dyn Problem>> = HashMap::new();
    // Github copilot tab+enter spam is faster than writing a macro lmao
    problems.insert(1, Box::new(problems::Day1));
    problems.insert(2, Box::new(problems::Day2));
    problems.insert(3, Box::new(problems::Day3));
    problems.insert(4, Box::new(problems::Day4));
    problems.insert(5, Box::new(problems::Day5));
    problems.insert(6, Box::new(problems::Day6));
    problems.insert(7, Box::new(problems::Day7));

    println!("Which day do you want to solve?");
    let mut day = String::new();
    std::io::stdin().read_line(&mut day).unwrap();
    let day = match day.trim().parse::<usize>() {
        Ok(day) => {
            if problems.contains_key(&day) {
                day
            } else {
                println!("Invalid problem number");
                return;
            }
        },
        Err(_) => {
            println!("Invalid problem number");
            return;
        }
    };

    println!("Which part do you want to solve?");
    let mut part = String::new();
    std::io::stdin().read_line(&mut part).unwrap();
    let part = match part.trim().parse::<usize>() {
        Ok(part ) => {
            if part == 1 || part == 2 {
                part
            } else {
                println!("Invalid problem part number");
                return;
            }
        },
        Err(_) => {
            println!("Invalid problem part number");
            return;
        }
    };

    let input = fs::read_to_string(format!("inputs/day{}", day)).expect("No input file for this problem");
    let problem = problems.get(&day).unwrap();

    println!("Problem {} part {} solution: {}", day, part, if part == 1 {problem.solve1(&input)} else {problem.solve2(&input)});
}
