use crate::problems::Problem;

pub struct Day1;

impl Problem for Day1 {
    fn solve1(&self, input: &str) -> String {
        let mut old = 0;
        let mut count = -1;
        for line in input.lines() {
            let new: u32 = line.trim().parse().expect("invalid number in input");
            if new > old {
                count += 1;
            }
            old = new;
        }
        format!("{}", count)
    }

    fn solve2(&self, input: &str) -> String {
        let mut old = 0;
        let mut count = -3;
        let mut a;
        let mut b = 0;
        let mut c = 0;
        for line in input.lines() {
            let new: u32 = line.trim().parse().expect("invalid number in input");
            a = b;
            b = c;
            c = new;
            if a + b + c > old {
                count += 1;
            }
            old = a + b + c;
        }
        format!("{}", count)
    }
}