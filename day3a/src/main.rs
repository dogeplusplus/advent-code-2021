use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot read input"));
    let mut counts = [0; 12];
    let mut num_records = 0;

    for line in reader.lines() {
        for (i, bit) in line.unwrap().chars().enumerate() {
            match bit {
                '1' => counts[i] += 1,
                '0' => (),
                _ => panic!("Could not read integer")
            }
        }
        num_records += 1;
    }

    let gamma = counts.map(|x| format!("{}", (x > (num_records / 2)) as u32)).join("");
    let epsilon = counts.map(|x| format!("{}", (x <= (num_records / 2)) as u32)).join("");
    let gamma = i32::from_str_radix(&gamma, 2).expect("Not binary");
    let epsilon = i32::from_str_radix(&epsilon, 2).expect("Not binary");


    println!("{}", gamma);
    println!("{}", epsilon);
    println!("{}", gamma * epsilon);
}
