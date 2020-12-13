use std::iter;
use crate::day::*;

pub struct Day13 {}

impl Day for Day13 {
    fn tag(&self) -> &str { "13" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day13 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let mut lines = io::BufReader::new(input).lines();
        let t0 = lines.next().unwrap()?.parse::<usize>().unwrap();
        let ids = lines.next().unwrap()?;
        let mut ids = ids.split(",").filter(|&id| id != "x").map(|id| id.parse::<usize>().unwrap());
        let (id, w) = ids.map(|id| (id, id - t0 % id)).min_by_key(|(_, w)| *w).unwrap();
        Ok(id * w)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let mut lines = io::BufReader::new(input).lines().skip(1);
        let ids = lines.next().unwrap()?;
        let mut ids = ids.split(",").enumerate().filter(|&(_, id)| id != "x").map(|(i, id)| (i, id.parse::<usize>().unwrap()));
        let s = ids.next().unwrap();
        Ok(ids.fold(s, |(m, k), (i, id)| {
            (iter::repeat(()).enumerate().map(|(x, _)| k * x + m).filter(|&t| (t + i) % id == 0).next().unwrap(), k * id)
        }).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day13 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("939
7,13,x,x,59,x,31,19", 295);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day13 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("
17,x,13,19", 3417);
        test2("
67,7,59,61", 754018);
        test2("
67,x,7,59,61", 779210);
        test2("
67,7,x,59,61", 1261476);
        test2("
1789,37,47,1889", 1202161486);
        test2("939
7,13,x,x,59,x,31,19", 1068781);
    }
}