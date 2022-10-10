mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

// Github Copilot go brrrrrrrrrrr
pub use day1::*;
pub use day2::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;
pub use day7::*;


pub trait Problem {
    fn solve1(&self, input: &str) -> String;
    fn solve2(&self, input: &str) -> String;
}