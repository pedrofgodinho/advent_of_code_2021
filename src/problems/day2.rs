use crate::problems::Problem;

pub struct Day2;

impl Problem for Day2 {
    fn solve1(&self, lines: &str) -> String {
        let mut horizontal = 0;
        let mut vertical = 0;
        for line in lines.lines(){
            if line.is_empty() {
                continue;
            }
            let mut split = line.split(' ');
            let mov = split.next().expect("invalid movement");
            let val: i32 = split.next().expect("invalid movement").parse().expect("invalid movement");
            match mov {
                "forward" => horizontal += val,
                "up" => vertical -= val,
                "down" => vertical += val,
                _ => panic!("invalid movement"),
            }
        }
        format!("{}", horizontal * vertical)
    }


    fn solve2(&self, input: &str) -> String {
        let mut horizontal = 0;
        let mut vertical = 0;
        let mut aim = 0;
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let mut split = line.split(' ');
            let mov = split.next().expect("invalid movement");
            let val: i32 = split.next().expect("invalid movement").parse().expect("invalid movement");
            match mov {
                "forward" => {
                    horizontal += val;
                    vertical += val * aim;
                },
                "up" => aim -= val,
                "down" => aim += val,
                _ => panic!("invalid movement"),
            }
        }
        format!("{}", horizontal * vertical)
    }
}