use std::str;
use crate::day::*;

pub struct Day12 {}

impl Day for Day12 {
    fn tag(&self) -> &str { "12" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day12 {
    fn travel(self: &Self, input: &mut dyn io::Read, d: (i32, i32), mode: bool) -> BoxResult<i32> {
        io::BufReader::new(input).lines().map(|r| r.unwrap()).fold(Ok(((0, 0), d)), |r, a| {
            let (p, d) = r?;
            let s = a.as_bytes();
            let n = str::from_utf8(&s[1..]).unwrap().parse::<i32>()?;
            fn rot(d: (i32, i32), a: i32) -> BoxResult<(i32, i32)> {
                match a % 360 / 90 {
                    0 => Ok(d),
                    1 => Ok((-d.1, d.0)),
                    2 => Ok((-d.0, -d.1)),
                    3 => Ok((d.1, -d.0)),
                    _ => Err(AocError.into())
                }
            }
            match s[0] {
                b'N' => Ok(if mode { ((p.0, p.1 + n), d) } else { (p, (d.0, d.1 + n)) }),
                b'S' => Ok(if mode { ((p.0, p.1 - n), d) } else { (p, (d.0, d.1 - n)) }),
                b'E' => Ok(if mode { ((p.0 + n, p.1), d) } else { (p, (d.0 + n, d.1)) }),
                b'W' => Ok(if mode { ((p.0 - n, p.1), d) } else { (p, (d.0 - n, d.1)) }),
                b'L' => Ok((p, rot(d, n)?)),
                b'R' => Ok((p, rot(d, 360 - (n % 360))?)),
                b'F' => Ok(((p.0 + n * d.0, p.1 + n * d.1), d)),
                _ => Err(AocError.into()),
            }
        }).map(|(p, _): ((i32, i32), _)| (p.0.abs() + p.1.abs()))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<i32> {
        self.travel(input, (1, 0), true)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<i32> {
        self.travel(input, (10, 1), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: i32) {
        assert_eq!(Day12 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("F10
N3
F7
R90
F11", 25);
    }

    fn test2(s: &str, f: i32) {
        assert_eq!(Day12 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("F10
N3
F7
R90
F11", 286);
    }
}