pub use itertools::Itertools;
pub use std::error;
use std::fmt;
pub use std::io;
pub use std::io::BufRead;

pub type BoxResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub struct AocError;

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected error")
    }
}

impl error::Error for AocError {}

pub trait Day {
    fn tag(&self) -> &str;
    fn part1(&self, _input: &dyn Fn() -> Box<dyn io::Read>) {}
    fn part2(&self, _input: &dyn Fn() -> Box<dyn io::Read>) {}

    fn numbers<'a>(&self, input: &'a mut dyn io::Read) -> Box<dyn Iterator<Item=BoxResult<i32>> + 'a> {
        let lines = io::BufReader::new(input).lines();
        Box::new(lines.map(|r| r.map_err(|e| e.into())
            .and_then(|s| s.parse::<i32>().map_err(|e| e.into()))))
    }
}