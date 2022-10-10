use std::collections::HashMap;
use crate::problems::Problem;

#[derive(Debug)]
struct Line {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Line {
    fn new(start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Line {
        Line {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    fn cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        let mut x = self.start_x;
        let mut y = self.start_y;

        cells.push((x, y));

        while x != self.end_x || y != self.end_y {
            if x < self.end_x {
                x += 1;
            } else if x > self.end_x {
                x -= 1;
            }
            if y < self.end_y {
                y += 1;
            } else if y > self.end_y {
                y -= 1;
            }
            cells.push((x, y));
        }
        cells
    }
}

pub struct Day5;

impl Day5 {
    fn parse_input(&self, input: &str) -> Vec<Line> {
        input.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut sides = line.split(" -> ");
                let mut start = sides.next().unwrap().split(',').map(|num| num.parse::<usize>().unwrap());
                let mut end = sides.next().unwrap().split(',').map(|num| num.parse::<usize>().unwrap());
                Line::new(start.next().unwrap(), start.next().unwrap(), end.next().unwrap(), end.next().unwrap())
            })
            .collect()
    }
}


impl Problem for Day5 {
    fn solve1(&self, input: &str) -> String {
        let lines: Vec<Line> = self.parse_input(input).into_iter().filter(|line| line.start_x == line.end_x || line.start_y == line.end_y).collect();

        let mut cells = HashMap::new();
        for line in lines {
            for point in line.cells() {
                let count = cells.entry(point).or_insert(0);
                *count += 1;
            }
        }
        format!("{}", cells.values().filter(|&count| *count > 1).count())
    }

    fn solve2(&self, input: &str) -> String {
        let lines: Vec<Line> = self.parse_input(input);

        let mut cells = HashMap::new();
        for line in lines {
            for point in line.cells() {
                let count = cells.entry(point).or_insert(0);
                *count += 1;
            }
        }
        format!("{}", cells.values().filter(|&count| *count > 1).count())
    }
}