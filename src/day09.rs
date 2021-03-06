use std::cmp::*;
use crate::day::*;

pub struct Day09 {}

impl Day for Day09 {
    fn tag(&self) -> &str { "09" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 25));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 25));
    }
}

impl Day09 {
    fn numbers(self: &Self, input: &mut dyn io::Read) -> BoxResult<Vec<i64>> {
        Utils::numbers(input).collect::<BoxResult<Vec<_>>>()
    }

    fn find_invalid(self: &Self, numbers: &Vec<i64>, p: usize) -> BoxResult<i64> {
        Ok(numbers.windows(p + 1).map(|w| {
            let n = *w.last().unwrap();
            if (&w[0..p]).iter().combinations(2).map(|c| c[0] + c[1]).find(|&s| s == n).is_some() { None } else { Some(n) }
        }).flatten().next().unwrap())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read, p: usize) -> BoxResult<i64> {
        self.find_invalid(&self.numbers(input)?, p)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, p: usize) -> BoxResult<i64> {
        let numbers = self.numbers(input)?;
        let n = self.find_invalid(&numbers, p)?;
        Ok((0..numbers.len() - 1).flat_map(|i| numbers.as_slice()[i..].iter().scan((0, None, None), |s, &x| {
            *s = (s.0 + x, Some(s.1.map_or(x, |m| min(x, m))), Some(s.2.map_or(x, |m| max(x, m))));
            Some(*s)
        }).skip(1).find(|(sum, _, _)| *sum >= n)).find(|(sum, _, _)| *sum == n)
        .map(|(_, min, max)| min.unwrap() + max.unwrap()).unwrap())
    }

    #[allow(dead_code)]
    fn part2_impl_naive(self: &Self, input: &mut dyn io::Read, p: usize) -> BoxResult<i64> {
        let numbers = self.numbers(input)?;
        let n = self.find_invalid(&numbers, p)?;
        Ok((0..numbers.len() - 1).map(|i| {
            let mut s = 0;
            let t = (i..numbers.len()).map(|j| { s = s + numbers[j]; (j, s) }).skip(1).find(|(_, s)| s >= &n).ok_or(AocError).unwrap();
            let r = &numbers.as_slice()[i..t.0 + 1];
            if t.1 == n { Some(r.iter().min().unwrap() + r.iter().max().unwrap()) } else { None }
        }).find(|o| o.is_some()).unwrap().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, p: usize, f: i64) {
        assert_eq!(Day09 {}.part1_impl(&mut s.as_bytes(), p).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576", 5, 127);
    }

    fn test2(s: &str, p: usize, f: i64) {
        assert_eq!(Day09 {}.part2_impl(&mut s.as_bytes(), p).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576", 5, 62);
    }
}