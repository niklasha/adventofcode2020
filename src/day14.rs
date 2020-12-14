use regex::Regex;
use std::collections::HashMap;
use crate::day::*;
use regex::internal::Input;

pub struct Day14 {}

impl Day for Day14 {
    fn tag(&self) -> &str { "14" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day14 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("^(mask = (.+))|(mem\\[(.+)\\] = (.+))$").unwrap();
        }
        let mut and_mask: usize = !0;
        let mut or_mask: usize = 0;
        let mut mem = HashMap::new();
        lines.for_each(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            if cap.get(1).is_some() {
                let mask = &cap[2];
                and_mask = usize::from_str_radix(&mask.clone().replace("X", "1"), 2).unwrap();
                or_mask = usize::from_str_radix(&mask.clone().replace("X", "0"), 2).unwrap();
            } else {
                mem.insert(cap[4].parse::<usize>().unwrap(), cap[5].parse::<usize>().unwrap() & and_mask | or_mask);
            }
        });
        Ok(mem.iter().filter(|&(_, &x)| x != 0).map(|(_, &x)| x).sum())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("^(mask = (.+))|(mem\\[(.+)\\] = (.+))$").unwrap();
        }
        let mut floating = Vec::new();
        let mut and_mask: usize = !0;
        let mut or_mask: usize = 0;
        let mut mem = HashMap::new();
        lines.for_each(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            if cap.get(1).is_some() {
                let mask = &cap[2];
                floating = mask.as_bytes().iter().enumerate().filter(|&(_, &c)| c == b'X').map(|(i, _)| 35 - i).collect::<Vec<_>>();
                and_mask = usize::from_str_radix(&mask.clone().replace("0", "1").replace("X", "0"), 2).unwrap();
                or_mask = usize::from_str_radix(&mask.clone().replace("X", "0"), 2).unwrap();
            } else {
                let a = cap[4].parse::<usize>().unwrap() & and_mask | or_mask;
                (0usize..1 << floating.len()).for_each(|x| {
                    mem.insert(
                        floating.iter().enumerate().fold(a, |a, (o, &i)|
                            if (x & (1 << o)) != 0 { a | 1 << i } else { a & !(1 << i) }),
                        cap[5].parse::<usize>().unwrap());
                })
            }
        });
        Ok(mem.iter().filter(|&(_, &x)| x != 0).map(|(_, &x)| x).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day14 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
", 165);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day14 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
", 208);
    }
}