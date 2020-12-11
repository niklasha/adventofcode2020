use std::cmp::*;
use crate::day::*;

pub struct Day11 {}

impl Day for Day11 {
    fn tag(&self) -> &str { "11" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day11 {
    fn count_distant_neighbours(seats: &Vec<Vec<u8>>, x0: usize, y0: usize) -> usize {
        let ymax = seats.len() as i32;
        let xmax = seats.first().unwrap().len() as i32;
        (-1i32..2).flat_map(|dy| {
            (-1i32..2).filter(move |&dx| !(dx == 0 && dy == 0)).filter(move |&dx| {
                let mut x = x0 as i32 + dx;
                let mut y = y0 as i32 + dy;
                while y >= 0 && y < ymax && x >= 0 && x < xmax && seats[y as usize][x as usize] == b'.' {
                    x += dx;
                    y += dy;
                }
                y >= 0 && y < ymax && x >= 0 && x < xmax && seats[y as usize][x as usize] == b'#'
            })
        }).count()
    }

    fn count_neighbours(seats: &Vec<Vec<u8>>, x0: usize, y0: usize) -> usize {
        (if y0 == 0 { 0 } else { y0 - 1 }..min(y0 + 2, seats.len())).flat_map(|y| {
            let row = &seats[y];
            (if x0 == 0 { 0 } else { x0 - 1 }..min(x0 + 2, row.len())).filter(move |&x| (x != x0 || y != y0) && row[x] == b'#')
        }).count()
    }

    fn occupied<F>(&self, seats0: &Vec<Vec<u8>>, count: F, limit: usize) -> BoxResult<usize>
    where F: Fn(&Vec<Vec<u8>>, usize, usize) -> usize {
        let ymax = seats0.len();
        let xmax = seats0.first().unwrap().len();
        let seats = &mut seats0.clone();
        let mut new = seats.clone();
        let mut done = false;
        while !done {
            done = true;
            (0..ymax).for_each(|y| {
                (0..xmax).for_each(|x| {
                    match seats[y][x] {
                        b'L' => if count(seats, x, y) == 0 { done = false; new[y][x] = b'#' },
                        b'#' => if count(seats, x, y) >= limit { done = false; new[y][x] = b'L' },
                        _ => ()
                    }
//                    print!("{}", new[y][x] as char);
                });
//                println!("");
            });
//            println!("");
            (0..ymax).for_each(|y|
                (0..xmax).for_each(|x| seats[y][x] = new[y][x]));
        }
        Ok(seats.iter().flat_map(|r| r.iter().filter(|&&s| s == b'#')).count())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.occupied(&Utils::byte_matrix(input).unwrap(), Self::count_neighbours, 4)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.occupied(&Utils::byte_matrix(input).unwrap(), Self::count_distant_neighbours, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day11 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL", 37);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day11 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL", 26);
    }
}