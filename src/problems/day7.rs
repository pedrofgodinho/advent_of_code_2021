use crate::problems::Problem;

pub struct Day7;

impl Problem for Day7 {
    fn solve1(&self, input: &str) -> String {
        let positions: Vec<u32> = input.trim().split(',').map(|pos| pos.parse::<u32>().unwrap()).collect();
        let max = *positions.iter().max().unwrap();
        let mut best_cost_position = (u32::MAX, u32::MAX);
        for i in 0..max {
            let mut cost = 0;
            for pos in &positions {
                cost += (*pos as i32 - i as i32).abs() as u32;
            }
            if cost < best_cost_position.0 {
                best_cost_position = (cost, i);
            }
        }
        format!("Position: {}  Fuel: {}", best_cost_position.1, best_cost_position.0)
    }

    fn solve2(&self, input: &str) -> String {
        let positions: Vec<u32> = input.trim().split(',').map(|pos| pos.parse::<u32>().unwrap()).collect();
        let max = *positions.iter().max().unwrap();
        let mut best_cost_position = (u32::MAX, u32::MAX);
        // 1+2+3+...+n = n(n+1)/2
        for i in 0..max {
            let mut cost = 0;
            for pos in &positions {
                let n = (*pos as i32 - i as i32).abs() as u32;
                cost += n * (n + 1) / 2;
            }
            if cost < best_cost_position.0 {
                best_cost_position = (cost, i);
            }
        }
        format!("Position: {}  Fuel: {}", best_cost_position.1, best_cost_position.0)
    }
}