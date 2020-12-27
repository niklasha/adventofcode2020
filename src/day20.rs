use crate::day::*;
use std::collections::{HashMap, HashSet};

pub struct Day20 {}

impl Day for Day20 {
    fn tag(&self) -> &str { "20" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 144));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 12, 2539));
    }
}

impl Day20 {
    fn corners(self: &Self, input: &mut dyn io::Read, n: usize) -> Vec<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        let mut edges = HashMap::new();
        let mut tiles = HashMap::new();
        for _i in 0..n {
            let mut tile = lines.take_while(|r| r.as_ref().unwrap() != "");
            let x = tile.next();
            let id = x.unwrap().unwrap().split_ascii_whitespace().nth(1).unwrap().strip_suffix(":").unwrap().parse::<usize>().unwrap();
            let tile = &tile.map(|r| r.unwrap().replace(".", "0").replace("#", "1")).collect_vec();
            let n = usize::from_str_radix(&tile[0], 2).unwrap();
            let s = usize::from_str_radix(tile.last().unwrap(), 2).unwrap();
            let e = usize::from_str_radix(&tile.iter().map(|s| s.chars().next().unwrap()).collect::<String>(), 2).unwrap();
            let w = usize::from_str_radix(&tile.iter().map(|s| s.chars().last().unwrap()).collect::<String>(), 2).unwrap();
            fn rev(x: &str) -> usize { usize::from_str_radix(&x.chars().rev().collect::<String>(), 2).unwrap() }
            let nr = rev(&tile[0]);
            let sr = rev(tile.last().unwrap());
            let er = rev(&tile.iter().map(|s| s.chars().next().unwrap()).collect::<String>());
            let wr = rev(&tile.iter().map(|s| s.chars().last().unwrap()).collect::<String>());
            fn ins(edges: &mut HashMap<usize, HashSet<usize>>, edge: usize, id: usize) {
                let mut s = edges.get_mut(&edge);
                if s == None {
                    edges.insert(edge, HashSet::new());
                    s = edges.get_mut(&edge);
                }
                s.unwrap().insert(id);
            }
            ins(&mut edges, n, id);
            ins(&mut edges, s, id);
            ins(&mut edges, e, id);
            ins(&mut edges, w, id);
            ins(&mut edges, nr, id);
            ins(&mut edges, sr, id);
            ins(&mut edges, er, id);
            ins(&mut edges, wr, id);
            // XXX why should we not add nr, sr, er, wr to the vec below?
            tiles.insert(id, vec![n, s, e, w]);
        }
        let uniques = edges.iter().filter(|&(_, ids)| ids.len() == 1).map(|(e, _)| e).collect::<HashSet<_>>();
        let cs = tiles.iter().filter(|&(_, es)| es.iter().filter(|&e| uniques.contains(e)).count() == 2).map(|(&id, _)| id).collect();
        println!("{:?}", cs);
        cs
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read, n: usize) -> BoxResult<usize> {
        Ok(self.corners(input, n).iter().product())
    }

    fn rot(tile: &Vec<String>) -> Vec<String> {
        let sz = tile.len();
        (0..sz).map(|y| (0..sz).map(move |x| tile[x].chars().collect_vec()[sz - y - 1]).collect::<String>()).collect()
    }

    fn flip(tile: &Vec<String>) -> Vec<String> {
        let sz = tile.len();
        (0..sz).map(|y| tile[sz - y - 1].clone()).collect()
    }

    fn configs(tile: &Vec<String>) -> HashSet<(Vec<String>, Vec<String>)> {
        let tile = tile.clone();
        let mut configs = HashSet::new();
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        let tile = Self::rot(&tile);
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        let tile = Self::rot(&tile);
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        let tile = Self::rot(&tile);
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        configs.iter().map(|c| {
            let edges = Self::edges(c);
            (c.clone(), edges)
        }).collect::<HashSet<_>>()
    }

    fn north(tile: &Vec<String>) -> String { tile[0].to_string() }
    fn south(tile: &Vec<String>) -> String { tile.last().unwrap().to_string() }
    fn west(tile: &Vec<String>) -> String { tile.iter().map(|r| r.chars().next().unwrap()).collect::<String>() }
    fn east(tile: &Vec<String>) -> String { tile.iter().map(|r| r.chars().last().unwrap()).collect::<String>() }

    const NORTH: usize = 0;
    const WEST: usize = 1;

    fn edges(tile: &Vec<String>) -> Vec<String> { vec![Self::north(tile), Self::west(tile), Self::south(tile), Self::east(tile)] }

    #[allow(dead_code)]
    fn print(tile: &Vec<String>) {
        for row in tile {
            println!("{}", row);
        }
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, n: usize, start: usize) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();

        let mut edges = HashMap::new();
        fn ins(edges: &mut HashMap<(String, usize), HashSet<(usize, Vec<String>)>>, edge: String, dir: usize, tile: &(usize, Vec<String>)) {
            let key = (edge, dir);
            let mut s = edges.get_mut(&key);
            if s == None {
                edges.insert(key.clone(), HashSet::new());
                s = edges.get_mut(&key);
            }
            s.unwrap().insert(tile.clone());
        }

        let mut tiles = HashMap::new();
        for _i in 0..(n * n) {
            let mut tile = lines.take_while(|r| r.as_ref().unwrap() != "");
            let id = tile.next().unwrap().unwrap().split_ascii_whitespace().nth(1).unwrap().strip_suffix(":").unwrap().parse::<usize>().unwrap();
            let tile = tile.map(|r| r.unwrap()).collect_vec();
            let configs = Self::configs(&tile);
            for (tile, es) in &configs {
                for dir in 0..4 {
                    let edge = es[dir].to_string();
                    ins(&mut edges, edge, dir, &(id, tile.clone()));
                }
            }
            tiles.insert(id, configs);
        }

        fn puzzle(path: Vec<(usize, Vec<String>)>,
                  tiles: HashMap<usize, HashSet<(Vec<String>, Vec<String>)>>,
                  n: usize,
                  edges: &HashMap<(String, usize), HashSet<(usize, Vec<String>)>>
        ) -> Option<Vec<(usize, Vec<String>)>> {
            if tiles.is_empty() { Some(path) }
            else {
                let i = path.len();
                let (r, c) = (i / n, i % n);
                let eastern = if c > 0 { edges.get(&(Day20::east(&path.last().unwrap().1), Day20::WEST)) } else { None };
                let southern = if r > 0 { edges.get(&(Day20::south(&path.get(i - n).unwrap().1), Day20::NORTH)) } else { None };
                let x = eastern.map_or(southern.clone().unwrap_or(&HashSet::new()).clone(),
                                       |e| southern.map_or(eastern.unwrap().clone(),
                                                           |s| e.iter().cloned().filter(|t| s.contains(t)).collect::<HashSet<_>>()));
                let x = x.iter().filter(|&(id, _)| !path.iter().any(|(path_id, _)| path_id == id)).cloned().collect_vec();
                x.iter().map(|candidate| {
                    let mut path = path.clone();
                    let mut t = tiles.clone();
                    t.remove(&candidate.0);
                    path.push(candidate.clone());
                    puzzle(path, t, n, edges)
                }).flatten().next()
            }
        }

        let mut t = tiles.clone();
        let map = t.remove(&start).unwrap().iter()
            .map(|config| puzzle(vec![(start, config.0.clone())], t.clone(), n, &edges))
            .flatten().next().unwrap();
        let map = map.chunks(n).map(|r| r.iter().map(|(_, t)| t.clone()).collect_vec()).collect_vec();
        let map = map.iter().flat_map(|rt|
            (1..rt[0].len() - 1).map(|i| rt.iter().map(|rs| (&rs[i][1..rs.len() - 1]).clone()).collect_vec().join("")).collect_vec()).collect_vec();
        let monster = vec![
            "                  # ".to_string(),
            "#    ##    ##    ###".to_string(),
            " #  #  #  #  #  #   ".to_string()
        ];
        let monster_sz = count_hashes(&monster);

        fn count_hashes(img: &Vec<String>) -> usize { img.iter().map(|s| s.chars().filter(|&c| c == '#').count()).sum() }

        fn matches(map: &Vec<String>, x: usize, y: usize, img: &Vec<String>) -> bool {
            let mut result = true;
            for dy in 0..img.len() {
                for dx in 0..img[0].len() {
                    if img[dy].chars().nth(dx).unwrap() == '#' && map[y + dy].chars().nth(x + dx).unwrap() != '#' { result = false }
                }
            }
            result
        }

        fn count(map: &Vec<String>, img: &Vec<String>) -> usize {
            (0..map.len() - img.len() + 1).map(|y|
                (0..map[0].len() - img[0].len() + 1).map(|x|
                    if matches(map, x, y, img) { 1usize } else { 0 }).sum::<usize>()).sum()
        }

        let maps = Self::configs(&map).iter().map(|(c, _)| c.clone()).collect_vec();
        let monster_count: usize = maps.iter().map(|map| count(map, &monster)).sum();
        Ok(count_hashes(&map) - monster_count * monster_sz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day20 {}.part1_impl(&mut s.as_bytes(), 9).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
", 20899048083289);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day20 {}.part2_impl(&mut s.as_bytes(), 3, 2971).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
", 273);
    }
}