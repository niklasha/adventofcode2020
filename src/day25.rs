use crate::day::*;

pub struct Day25 {}

impl Day for Day25 {
    fn tag(&self) -> &str { "25" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }
}

impl Day25 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<i64> {
        let mut numbers = Utils::numbers(input);
        let card_pk = numbers.next().unwrap().unwrap();
        let door_pk = numbers.next().unwrap().unwrap();
        let mut v = 1;
        let n = (1..20201227).find(|_| {
            v = (v * 7) % 20201227;
            v == card_pk || v == door_pk
        }).unwrap();
        Ok(Self::transform(if v == card_pk { door_pk } else { card_pk }, n))
    }

    fn transform(sn: i64, n: usize) -> i64 {
        let mut v = 1;
        for _ in 0..n {
            v = (v * sn) % 20201227;
        }
        v
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: i64) {
        assert_eq!(Day25 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("5764801
17807724
", 14897079);
    }
}