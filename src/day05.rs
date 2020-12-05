use crate::day::*;

pub struct Day05 {}

impl Day for Day05 {
    fn tag(&self) -> &str { "05" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day05 {
    fn process<'a>(self: &Self, input: &'a mut dyn io::Read) -> impl Iterator<Item=BoxResult<usize>> + 'a {
        io::BufReader::new(input).lines().map(|rs| rs.map_err(|e| e.into())
            .and_then(|s| usize::from_str_radix(
                &s.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1"), 2)
                .map_err(|e| e.into())))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input).map(Result::unwrap).max().ok_or(AocError.into())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input).map(Result::unwrap).sorted().tuple_windows().find(|(a, b)| b - a == 2).map(|(a, _)| a + 1).ok_or(AocError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day05 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL", 820);
    }
}