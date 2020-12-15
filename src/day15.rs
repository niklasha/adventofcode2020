use crate::day::*;
use std::collections::HashMap;

pub struct Day15 {}

impl Day for Day15 {
    fn tag(&self) -> &str { "15" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day15 {
    fn process(self: &Self, input: &mut dyn io::Read, n: usize) -> BoxResult<usize> {
        match io::BufReader::new(input).lines().next() {
            Some(Ok(s)) =>
                s.split(",").map(|n| Ok(n.parse::<usize>()?)).collect::<BoxResult<Vec<_>>>().map(|v| {
                    let mut seen = HashMap::new();
                    (1..n).fold(v[0], |l, i| {
                        let x = if i < v.len() { v[i] }
                        else {
                            if seen.contains_key(&l) {
                                i - seen.get(&l).unwrap()
                            } else { 0 }
                        };
                        seen.insert(l, i);
                        x
                    })
                }),
            _ => Err(AocError.into())
        }
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input, 2020)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input, 30000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day15 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("0,3,6", 436);
        test1("1,3,2", 1);
        test1("2,1,3", 10);
        test1("1,2,3", 27);
        test1("2,3,1", 78);
        test1("3,2,1", 438);
        test1("3,1,2", 1836);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day15 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("0,3,6", 175594);
        test2("1,3,2", 2578);
        test2("2,1,3", 3544142);
        test2("1,2,3", 261214);
        test2("2,3,1", 6895259);
        test2("3,2,1", 18);
        test2("3,1,2", 362);
    }
}