use crate::problems::Problem;

pub struct Day3;

impl Problem for Day3 {
    fn solve1(&self, input: &str) -> String {
        let mut bits = Vec::with_capacity(8);
        let mut count = 0;
        for line in input.lines() {
            if bits.len() < line.len() {
                bits.resize(line.len(), 0);
            }
            for (index, bit) in line.chars().enumerate() {
                if bit == '1' {
                    bits[index] += 1;
                }
            }
            count += 1;
        }

        let mut gamma = String::new();
        let mut epsilon = String::new();
        for bit_count in bits {
            if bit_count > count / 2 {
                gamma.push('1');
                epsilon.push('0');
            } else {
                gamma.push('0');
                epsilon.push('1');
            }
        }

        let gamma = isize::from_str_radix(&gamma, 2).unwrap();
        let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

        format!("{}", gamma * epsilon)
    }

    fn solve2(&self, input: &str) -> String {
        let vec: Vec<&str> = input.lines().into_iter().map(|item| item.trim()).filter(|item| !item.is_empty()).collect();

        let mut bits1: Vec<Vec<char>> = vec.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        let mut bit_index = 0;
        while bits1.len() != 1 {
            let bit_count = bits1.iter().fold(vec![0; bits1[0].len()], |mut acc, bit| {
                for (index, bit) in bit.iter().enumerate() {
                    if *bit == '1' {
                        acc[index] += 1;
                    }
                }
                acc
            });
            let wanted_bit = if bits1.len() - bit_count[bit_index] == bit_count[bit_index] || bit_count[bit_index] > bits1.len() - bit_count[bit_index] {
                '1'
            } else {
                '0'
            };
            bits1 = bits1.into_iter().filter(|number| number[bit_index] == wanted_bit).collect();
            bit_index += 1;
        }

        let mut bits2: Vec<Vec<char>> = vec.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        let mut bit_index = 0;
        while bits2.len() != 1 {
            let bit_count = bits2.iter().fold(vec![0; bits2[0].len()], |mut acc, bit| {
                for (index, bit) in bit.iter().enumerate() {
                    if *bit == '1' {
                        acc[index] += 1;
                    }
                }
                acc
            });
            let wanted_bit = if bits2.len() - bit_count[bit_index] == bit_count[bit_index] || bit_count[bit_index] > bits2.len() - bit_count[bit_index] {
                '0'
            } else {
                '1'
            };
            bits2 = bits2.into_iter().filter(|number| number[bit_index] == wanted_bit).collect();
            bit_index += 1;
        }

        let oxygen = isize::from_str_radix(&bits1[0].iter().collect::<String>(), 2).unwrap();
        let co2 = isize::from_str_radix(&bits2[0].iter().collect::<String>(), 2).unwrap();
        format!("{}", oxygen * co2)
    }
}