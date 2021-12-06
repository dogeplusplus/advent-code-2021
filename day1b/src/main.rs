use std::fs::File;
use itertools::Itertools;
use std::io::{BufReader, BufRead, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}



pub fn main() {
    let nums = read(File::open("day1.txt").unwrap()).unwrap();
    let nums = nums.into_iter();

    let mut increase = 0;
    let mut previous = 1000000000000000000;
    for (one, two, three) in nums.tuple_windows() {
        increase += (one + two + three > previous) as u32;
        previous = one + two + three;
    }
    println!("{}", increase);
}
