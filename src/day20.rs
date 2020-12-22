use crate::day::*;
use std::collections::{HashMap, HashSet};

pub struct Day20 {}

impl Day for Day20 {
    fn tag(&self) -> &str { "20" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 144));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 12, 0));
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
        (0..sz).map(|y| (0..sz).map(move |x| tile[x].chars().collect_vec()[y]).collect::<String>()).collect()
    }

    fn flip(tile: &Vec<String>) -> Vec<String> {
        let sz = tile.len();
        (0..sz).map(|y| tile[sz - y - 1].clone()).collect()
    }

    fn configs(tile: &Vec<String>) -> HashSet<(Vec<String>, Vec<String>)> {
        let mut tile = tile.clone();
        let mut configs = HashSet::new();
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        tile = Self::rot(&tile);
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        tile = Self::rot(&tile);
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        tile = Self::rot(&tile);
        configs.insert(tile.clone());
        configs.insert(Self::flip(&tile));
        configs.iter().map(|c| {
            let edges = Self::edges(c);
            (c.clone(), edges)
        }).collect::<HashSet<_>>()
    }

    fn north(tile: &Vec<String>) -> String { tile[0].to_string() }
    fn south(tile: &Vec<String>) -> String { tile.iter().last().unwrap().to_string() }
    fn west(tile: &Vec<String>) -> String { tile.iter().map(|r| r.chars().next().unwrap()).collect::<String>() }
    fn east(tile: &Vec<String>) -> String { tile.iter().map(|r| r.chars().last().unwrap()).collect::<String>() }

    fn edges(tile: &Vec<String>) -> Vec<String> { vec![ Self::north(tile), Self::west(tile), Self::south(tile), Self::east(tile) ] }

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
//        println!("{:?}", tiles);
//        println!("{:?}", edges);
        let (NORTH, WEST, SOUTH, EAST) = (0, 1, 2, 3);
        let unused = tiles.iter().map(|(&id, _)| id).collect::<HashSet<_>>();
        for config_0 in tiles.get(&start).unwrap().iter().map(|(x, _)| (start, x)) {
            let e0 = Self::east(config_0.1);
            let s0 = Self::south(config_0.1);
            println!("0 {:?} e {} s {}", config_0, e0, s0);
            if let Some(e_configs) = edges.get(&(e0, WEST)) {
                for config_1 in e_configs {
                    let e1 = Self::east(&config_1.1);
                    let s1 = Self::south(&config_1.1);
                    println!("1 {:?} e {} s {}", config_1.0, e1, s1);
                    if let Some(e_configs) = edges.get(&(e1, WEST)) {
                        for config_2 in e_configs {
                            let s2 = Self::south(&config_2.1);
                            println!("2 {:?} s {}", config_2.0, s2);
                            if let Some(s_configs) = edges.get(&(s0.clone(), NORTH)) {
                                for config_3 in s_configs {
                                    let e3 = Self::east(&config_3.1);
                                    let s3 = Self::south(&config_3.1);
                                    println!("3 {:?} e {} s {}", config_3.0, e3, s3);
                                    if let Some(e_configs) = edges.get(&(e3, WEST)) {
                                        if let Some(s_configs) = edges.get(&(s1.clone(), NORTH)) {
                                            println!("s_configs {:?}", s_configs);
                                            for config_4 in e_configs.intersection(s_configs) {
                                                let e4 = Self::east(&config_4.1);
                                                let s4 = Self::south(&config_4.1);
                                                println!("4 {:?} e {} s {}", config_4.0, e4, s4);
                                                if let Some(e_configs) = edges.get(&(e4, WEST)) {
                                                    if let Some(s_configs) = edges.get(&(s2.clone(), NORTH)) {
                                                        println!("s_configs {:?}", s_configs);
                                                        for config_5 in e_configs.intersection(s_configs) {
                                                            let s5 = Self::south(&config_5.1);
                                                            println!("5 {:?} s {}", config_5.0, s5);
                                                            if let Some(s_configs) = edges.get(&(s3.clone(), NORTH)) {
                                                                for config_6 in s_configs {
                                                                    let e6 = Self::east(&config_6.1);
                                                                    println!("6 {:?} e {}", config_6.0, e6);
                                                                    if let Some(e_configs) = edges.get(&(e6, WEST)) {
                                                                        if let Some(s_configs) = edges.get(&(s4.clone(), NORTH)) {
                                                                            println!("s_configs {:?}", s_configs);
                                                                            for config_7 in e_configs.intersection(s_configs) {
                                                                                let e7 = Self::east(&config_7.1);
                                                                                println!("7 {:?} e {}", config_7.0, e7);
                                                                                if let Some(e_configs) = edges.get(&(e7, WEST)) {
                                                                                    if let Some(s_configs) = edges.get(&(s5.clone(), NORTH)) {
                                                                                        println!("s_configs {:?}", s_configs);
                                                                                        for config_8 in e_configs.intersection(s_configs) {
                                                                                            println!("LAST {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", config_0.0, config_1.0, config_2.0, config_3.0, config_4.0, config_5.0, config_6.0, config_7.0, config_8.0);
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let perms = tiles.iter().permutations(n * n).map(|perm| {
            perm.iter().map(|&(_, tile)| tile).collect_vec()
        });
        let perms = perms.filter(|perm|
            (0..(n * n) - 1).all(|i| {
                let h = (i + 1) % n == n - 1 ||
                    perm[i + 1].iter().any(|(_, b_edges)| perm[i].iter().any(|(_, a_edges)| a_edges.iter().any(|edge| b_edges.contains(edge))));
                let v = i + n >= (n * (n - 1)) ||
                    perm[i + n].iter().any(|(_, c_edges)| perm[i].iter().any(|(_, a_edges)| a_edges.iter().any(|edge| c_edges.contains(edge))));
                h && v
            }));
        //let x = perms.next().unwrap();
//        let x = perms.count();
        let x = perms.take(1).collect_vec();
        println!("{:?}", x);
        Ok(0)
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