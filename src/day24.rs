use crate::day::*;
use std::collections::HashMap;

pub struct Day24 {}

impl Day for Day24 {
    fn tag(&self) -> &str { "24" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day24 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<usize> {
        Ok(Self::read_grid(input).iter().filter(|&(_, t)| *t).count())
    }

    fn read_grid(input: &mut dyn io::Read) -> HashMap<(i32, i32), bool> {
        let lines = io::BufReader::new(input).lines().map(|r|
            r.unwrap().chars().map(|c| c.to_string()).coalesce(|prev, curr|
                if prev == "n" || prev == "s" { Ok(format!("{}{}", prev, curr)) } else { Err((prev, curr)) }).collect_vec());
        let mut grid = HashMap::<(i32, i32), bool>::new();
        let delta: HashMap<String, (i32, i32)> = vec![("nw", (-1, -1)), ("w", (-2, 0)), ("sw", (-1, 1)), ("se", (1, 1)), ("e", (2, 0)), ("ne", (1, -1))]
            .iter().map(|(s, d)| ((*s).to_string(), *d)).collect();
        lines.for_each(|line| {
            let p = line.iter().fold((0, 0), |(x, y), dir| {
                let (dx, dy) = delta.get(dir).unwrap();
                (x + dx, y + dy)
            });
            let tile = grid.get(&p).unwrap_or(&false);
            grid.insert(p, !*tile);
        });
        grid
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let grid = Day24::read_grid(input);
        let delta: HashMap<String, (i32, i32)> = vec![("nw", (-1, -1)), ("w", (-2, 0)), ("sw", (-1, 1)), ("se", (1, 1)), ("e", (2, 0)), ("ne", (1, -1))]
            .iter().map(|(s, d)| ((*s).to_string(), *d)).collect();
        Ok((0..100).fold(grid, |grid, _| {
            let xs = grid.keys().map(|&(x, _)| x).collect_vec();
            let ys = grid.keys().map(|&(_, y)| y).collect_vec();
            let (xmin, xmax, ymin, ymax) =
                (xs.iter().min().unwrap() - 2, xs.iter().max().unwrap() + 2, ys.iter().min().unwrap() - 1, ys.iter().max().unwrap() + 1);
            let mut next = HashMap::<(i32, i32), bool>::new();
            for x in xmin..xmax + 1 {
                for y in ymin..ymax + 1 {
                    let n = delta.values()
                        .map(|&(dx, dy)| *grid.get(&(x + dx, y + dy)).unwrap_or(&false))
                        .filter(|&t| t).count();
                    let t = *grid.get(&(x, y)).unwrap_or(&false);
                    if t && !(n == 0 || n > 2) || n == 2 { next.insert((x, y), true); }
                }
            }
            next
        }).iter().filter(|&(_, t)| *t).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day24 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew", 10);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day24 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew", 2208);
    }
}