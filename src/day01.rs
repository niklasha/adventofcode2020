use crate::day::*;

pub struct Day01 {}

impl Day for Day01 {
    fn tag(&self) -> &str { "01" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day01 {
    fn product_of_tuple_with_correct_sum(self: &Self, input: &mut dyn io::Read, n: usize, sum: i32) -> BoxResult<i32> {
        self.numbers(input).map(Result::unwrap).combinations(n)
            .find_map(|v| if v.iter().sum::<i32>() == sum { Some(v.iter().product::<i32>()) } else { None })
            .ok_or(Box::new(AocError))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<i32> {
        self.product_of_tuple_with_correct_sum(input, 2, 2020)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<i32> {
        self.product_of_tuple_with_correct_sum(input, 3, 2020)
    }

    #[allow(dead_code)]
    fn part1_impl_naive(self: &Self, input: &mut dyn io::Read) -> BoxResult<i32> {
        let reader = io::BufReader::new(input);
        let v = reader.lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
        for (i, el1) in v.iter().enumerate() {
            for el2 in v[i + 1..].iter() {
                if el1 + el2 == 2020 {
                    return Ok(el1 * el2)
                }
            }
        }
        Err(Box::new(AocError))
    }

    #[allow(dead_code)]
    fn part2_impl_naive(self: &Self, input: &mut dyn io::Read) -> BoxResult<i32> {
        let reader = io::BufReader::new(input);
        let v = reader.lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
        for (i, el1) in v.iter().enumerate() {
            for (j, el2) in v[i + 1..].iter().enumerate() {
                for el3 in v[i + j + 1..].iter() {
                    if el1 + el2 + el3 == 2020 {
                        return Ok(el1 * el2 * el3);
                    }
                }
            }
        }
        Err(Box::new(AocError))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: i32) {
        assert_eq!(Day01 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("1721
979
366
299
675
1456", 514579);
    }

    fn test2(s: &str, f: i32) {
        assert_eq!(Day01 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("1721
979
366
299
675
1456", 241861950);
    }
}