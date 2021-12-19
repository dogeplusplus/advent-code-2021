extern crate ndarray;

use ndarray::Array2;
use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut vents = Array2::<u32>::zeros((1000, 1000));

    for line in reader.lines() {
        let line = line?;
        let coordinates: Vec<&str> = line.split(" -> ").collect();
        let source: Vec<usize> = coordinates[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let target: Vec<usize> = coordinates[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let x1 = if source[0] < target[0] { source[0] } else { target[0] };
        let x2 = if source[0] < target[0] { target[0] } else { source[0] };
        let y1 = if source[1] < target[1] { source[1] } else { target[1] };
        let y2 = if source[1] < target[1] { target[1] } else { source[1] };

        if x1 != x2 && y1 != y2 {
            continue;
        }

        for i in x1..x2+1 {
            for j in y1..y2+1 {
                vents[[i,j]] += 1;
            }
        }
    }

    let overlaps = vents.mapv(|a| (a > 1) as u32).sum();
    println!("{}", overlaps);
    Ok(())
}
