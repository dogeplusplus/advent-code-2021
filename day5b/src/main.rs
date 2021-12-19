extern crate ndarray;

use ndarray::Array2;
use itertools::{Itertools,EitherOrBoth::*};
use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut vents = Array2::<u32>::zeros((1000, 1000));

    for line in reader.lines() {
        let line = line?;
        let coordinates: Vec<&str> = line.split(" -> ").collect();
        let source: Vec<i32> = coordinates[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let target: Vec<i32> = coordinates[1].split(",").map(|x| x.parse::<i32>().unwrap()).collect();

        let x1 = source[0];
        let x2 = target[0];
        let y1 = source[1];
        let y2 = target[1];

        if x1 != x2 && y1 != y2 && (y2-y1).abs() != (x2-x1).abs() {
            continue;
        }

        let x_range: Vec<i32> = if x1 <= x2 { (x1..=x2).collect() } else { (x2..=x1).rev().collect() };
        let y_range: Vec<i32> = if y1 <= y2 { (y1..=y2).collect() } else { (y2..=y1).rev().collect() };

        for pair in x_range.iter().zip_longest(y_range.iter()) {
            match pair {
                Both(x, y) => vents[[*x as usize, *y as usize]] += 1,
                Left(x) => vents[[*x as usize, y1 as usize]] += 1,
                Right(y) => vents[[x1 as usize, *y as usize]] += 1,
            }
        }
    }

    let overlaps = vents.mapv(|a| (a > 1) as u32).sum();
    println!("{}", overlaps);
    Ok(())
}
