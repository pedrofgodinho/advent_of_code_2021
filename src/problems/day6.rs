use crate::problems::Problem;

pub struct Day6;

impl Day6 {
    fn simulate(&self, input: &str, iterations: usize) -> u128 {
        let mut fishes_per_state: [u128; 9] = [0; 9];
        input.trim().split(',').map(|num| num.parse::<usize>().unwrap()).for_each(|fish| fishes_per_state[fish] += 1);

        for _ in 0..iterations {
            let new_fish = fishes_per_state[0];
            for i in 1..9 {
                fishes_per_state[i - 1] = fishes_per_state[i];
            }
            fishes_per_state[8] = new_fish;
            fishes_per_state[6] += new_fish;
        }
        fishes_per_state.into_iter().sum()
    }
}

impl Problem for Day6 {
    fn solve1(&self, input: &str) -> String {
        format!("{}", self.simulate(input, 80))
    }


    fn solve2(&self, input: &str) -> String {
        format!("{}", self.simulate(input, 256))
    }
}