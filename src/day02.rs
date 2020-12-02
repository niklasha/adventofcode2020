use regex::Regex;
use crate::day::*;

pub struct Day02 {}

impl Day for Day02 {
    fn tag(&self) -> &str { "02" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day02 {
    fn valid_passwords(self: &Self, input: &mut dyn io::Read, is_pos: bool) -> BoxResult<usize> {
        let lines = io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+)-(.+) (.): (.+)").unwrap();
        }
        Ok(lines.map(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            let n1 = cap[1].parse::<usize>().unwrap();
            let n2 = cap[2].parse::<usize>().unwrap();
            let c: &str = &cap[3];
            let pwd: &str = &cap[4];
            if is_pos {
                (&pwd[n1 - 1..n1] == c) != (&pwd[n2 - 1..n2] == c)
            } else {
                let cnt = pwd.matches(c).count();
                cnt >= n1 && cnt <= n2
            }
        }).filter(|&b| b).count())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.valid_passwords(input, false)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.valid_passwords(input, true)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day02 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc", 2);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day02 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc", 1);
    }
}