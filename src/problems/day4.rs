use itertools::Itertools;
use crate::problems::Problem;

#[derive(Copy, Clone, Debug)]
struct BingoCell {
    value: u32,
    is_marked: bool,
}

#[derive(Copy, Clone, Debug)]
struct Bingo {
    cells: [BingoCell; 25],
    last_marked: Option<u32>,
}

impl Bingo {
    fn new(numbers: [u32; 25]) -> Bingo {
        let mut cells = [BingoCell { value: 0, is_marked: false }; 25];
        for (i, cell) in cells.iter_mut().enumerate() {
            cell.value = numbers[i];
        }
        Bingo {
            cells,
            last_marked:  None,
        }
    }

    fn mark(&mut self, number: u32) -> bool {
        if self.is_solved() {
            return false;
        }
        if let Some(cell) = self.cells.iter_mut().find(|cell| cell.value == number) {
            cell.is_marked = true;
            self.last_marked = Some(number);
            return true;
        }
        return false;
    }

    fn is_solved(&self) -> bool {
        let mut rows = [0; 5];
        let mut columns = [0; 5];
        let diagonals = [0; 2];
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_marked {
                rows[i / 5] += 1;
                columns[i % 5] += 1;
            }
        }
        rows.iter().any(|&count| count == 5) || columns.iter().any(|&count| count == 5) || diagonals.iter().any(|&count| count == 5)
    }

    fn get_score(&self) -> u32 {
        self.cells.iter()
            .filter(|cell| !cell.is_marked)
            .fold(0, |acc, cell| acc + cell.value) * self.last_marked.unwrap()
    }
}

pub struct Day4;

impl Day4 {
    fn parse_input(&self, input: &str) -> (Vec<u32>, Vec<Bingo>) {
        let mut lines = input.lines().map(|line| line.trim()).filter(|line| !line.is_empty());

        let numbers: Vec<u32> = lines.next().unwrap().split(',').map(|num| num.parse::<u32>().unwrap()).collect();

        let bingos: Vec<Bingo> = lines.map(|line| line.split_whitespace())
            .flatten()
            .map(|num| num.trim().parse::<u32>().unwrap())
            .chunks(25)
            .into_iter()
            .map(|chunk| Bingo::new(chunk.collect::<Vec<u32>>().try_into().unwrap()))
            .collect();
        (numbers, bingos)
    }
}

impl Problem for Day4 {
    fn solve1(&self, input: &str) -> String {
        let (numbers, mut bingos) = self.parse_input(input);

        for number in numbers {
            for bingo in bingos.iter_mut() {
                if bingo.mark(number) && bingo.is_solved() {
                    return format!("{}", bingo.get_score());
                }
            }
        }
        return "No solution found".to_string();
    }

    fn solve2(&self, input: &str) -> String {
        let (numbers, mut bingos) = self.parse_input(input);

        let mut solution = "No solution found".to_string();

        for number in numbers {
            for bingo in bingos.iter_mut() {
                if bingo.mark(number) && bingo.is_solved() {
                    solution = format!("{}", bingo.get_score());
                }
            }
        }
        return solution;
    }
}