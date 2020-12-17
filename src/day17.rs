use closure::closure;
use crate::day::*;
use itertools::Itertools;
use std::cmp::*;
use std::collections::HashSet;

pub struct Day17 {}

impl Day for Day17 {
    fn tag(&self) -> &str { "17" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 6));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 6));
    }
}

impl Day17 {
    fn count1(self: &Self, grid: &Vec<Vec<u8>>, n: usize) -> BoxResult<usize> {
        let map = grid.iter().enumerate().flat_map(|(y, row)|
            row.iter().enumerate().flat_map(move |(x, cube)| match cube {
                b'#' => Some((x as i64, y as i64, 0)),
                _ => None,
            })).collect::<HashSet<(i64, i64, i64)>>();
        let neighbours = &(-1..2).flat_map(|x| (-1..2).flat_map(move |y| (-1..2).flat_map(move |z|
            if x == 0 && y == 0 && z == 0 { None } else { Some((x, y, z)) }))).collect_vec();
        let r = (0..n).fold(map, |map, _| {
            let xmin = map.iter().fold(0i64, |m, (x, _, _)| min(m, *x)) - 1;
            let ymin = map.iter().fold(0i64, |m, (_, y, _)| min(m, *y)) - 1;
            let zmin = map.iter().fold(0i64, |m, (_, _, z)| min(m, *z)) - 1;
            let xmax = map.iter().fold(0i64, |m, (x, _, _)| max(m, *x)) + 1;
            let ymax = map.iter().fold(0i64, |m, (_, y, _)| max(m, *y)) + 1;
            let zmax = map.iter().fold(0i64, |m, (_, _, z)| max(m, *z)) + 1;
            (xmin..xmax + 1).flat_map(|x| (ymin..ymax + 1).flat_map(closure!(ref map, |y| (zmin..zmax + 1).flat_map(move |z| {
                let cnt = neighbours.iter().filter(|(dx, dy, dz)| map.contains(&(x + dx, y + dy, z + dz))).count();
                if map.contains(&(x, y, z)) && (cnt == 2 || cnt == 3) || !map.contains(&(x, y, z)) && cnt == 3 { Some((x, y, z)) } else { None }
            })))).collect::<HashSet<_>>()
        });
        Ok(r.len())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read, n: usize) -> BoxResult<usize> {
        self.count1(&Utils::byte_matrix(input).unwrap(), n)
    }

    fn count2(self: &Self, grid: &Vec<Vec<u8>>, n: usize) -> BoxResult<usize> {
        let map = grid.iter().enumerate().flat_map(|(y, row)|
            row.iter().enumerate().flat_map(move |(x, cube)| match cube {
                b'#' => Some((x as i64, y as i64, 0, 0)),
                _ => None,
            })).collect::<HashSet<(i64, i64, i64, i64)>>();
        let neighbours = &(-1..2).flat_map(|x| (-1..2).flat_map(move |y| (-1..2).flat_map(move |z| (-1..2).flat_map(move |w|
            if x == 0 && y == 0 && z == 0 && w == 0 { None } else { Some((x, y, z, w)) })))).collect_vec();
        let r = (0..n).fold(map, |map, _| {
            let xmin = map.iter().fold(0i64, |m, (x, _, _, _)| min(m, *x)) - 1;
            let ymin = map.iter().fold(0i64, |m, (_, y, _, _)| min(m, *y)) - 1;
            let zmin = map.iter().fold(0i64, |m, (_, _, z, _)| min(m, *z)) - 1;
            let wmin = map.iter().fold(0i64, |m, (_, _, _, w)| min(m, *w)) - 1;
            let xmax = map.iter().fold(0i64, |m, (x, _, _, _)| max(m, *x)) + 1;
            let ymax = map.iter().fold(0i64, |m, (_, y, _, _)| max(m, *y)) + 1;
            let zmax = map.iter().fold(0i64, |m, (_, _, z, _)| max(m, *z)) + 1;
            let wmax = map.iter().fold(0i64, |m, (_, _, _, w)| max(m, *w)) + 1;
            (xmin..xmax + 1).flat_map(|x| (ymin..ymax + 1).flat_map(closure!(ref map, |y| (zmin..zmax + 1).flat_map(move |z| (wmin..wmax + 1).flat_map(move |w| {
                let cnt = neighbours.iter().filter(|(dx, dy, dz, dw)| map.contains(&(x + dx, y + dy, z + dz, w + dw))).count();
                if map.contains(&(x, y, z, w)) && (cnt == 2 || cnt == 3) || !map.contains(&(x, y, z, w)) && cnt == 3 { Some((x, y, z, w)) } else { None }
            }))))).collect::<HashSet<_>>()
        });
        Ok(r.len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, n: usize) -> BoxResult<usize> {
        self.count2(&Utils::byte_matrix(input).unwrap(), n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day17 {}.part1_impl(&mut s.as_bytes(), 6).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(".#.
..#
###", 112);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day17 {}.part2_impl(&mut s.as_bytes(), 6).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(".#.
..#
###", 848);
    }
}