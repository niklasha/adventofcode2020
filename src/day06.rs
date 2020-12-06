use reduce::Reduce;
use std::collections::HashSet;
use crate::day::*;

pub struct Day06 {}

impl Day for Day06 {
    fn tag(&self) -> &str { "06" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day06 {
    fn process<'a, F>(self: &Self, input: &'a mut dyn io::Read, f: F) -> BoxResult<usize>
    where F: Fn(&str) -> usize {
        let declarations = io::BufReader::new(input).lines().map(Result::unwrap)
            .coalesce(|x, y| if y.is_empty() { Err((x, y)) } else { Ok(if x.is_empty() { y } else { format!("{} {}", x, y) }) });
        Ok(declarations.map(|s| f(s.as_str())).sum())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input, |s| s.chars().filter(|&c| c != ' ').unique().count())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input,
                     |s| s.split(' ').map(|s| s.chars().collect::<HashSet<_>>())
                         .reduce(|a, b| a.intersection(&b).map(|c| *c).collect()).unwrap().len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day06 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("abc

a
b
c

ab
ac

a
a
a
a

b", 11);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day06 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("abc

a
b
c

ab
ac

a
a
a
a

b", 6);
    }
}