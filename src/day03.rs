use crate::day::*;

pub struct Day03 {}

impl Day for Day03 {
    fn tag(&self) -> &str { "03" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day03 {
    fn tree_count(self: &Self, forest: &Vec<Vec<u8>>, slope: (usize, usize)) -> BoxResult<usize> {
        Ok((0..forest.len()).step_by(slope.1).fold((0, 0), |(x, c), y| {
            let row = &forest[y];
            (x + slope.0, c + if row[x % row.len()] == b'#' { 1 } else { 0 })
        }).1)
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.tree_count(&Utils::byte_matrix(input).unwrap(), (3, 1))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let forest = Utils::byte_matrix(input).unwrap();
        Ok(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|&s|
            self.tree_count(&forest, s).unwrap()).product())
    }

    #[allow(dead_code)]
    fn tree_count_naive(self: &Self, lines: &Vec<String>, slope: (usize, usize)) -> BoxResult<usize> {
        Ok(lines.iter().fold((0, 0, 0), |(c, x, y), l| {
            let s = l.chars().collect::<Vec<_>>();
            let x = x % s.len();
            (c + (if y == 0 && s[x] == '#' { 1 } else { 0 }), x + (if y == 0 { slope.0 } else { 0 }), (y + 1) % slope.1)
        }).0)
    }

    #[allow(dead_code)]
    fn part1_impl_naive(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = io::BufReader::new(input).lines().map(|rs| rs.unwrap()).collect::<Vec<_>>();
        self.tree_count_naive(&lines, (3, 1))
    }

    #[allow(dead_code)]
    fn part2_impl_naive(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = io::BufReader::new(input).lines().map(|rs| rs.unwrap()).collect::<Vec<_>>();
        Ok(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|&s|
            self.tree_count_naive(&lines, s).unwrap()).product())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day03 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#", 7);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day03 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#", 336);
    }
}