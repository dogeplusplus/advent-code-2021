use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot read input"));
    let mut depth = 0;
    let mut horizontal = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let magnitude = words[1].parse::<u32>().unwrap();

        match words[0] {
            "forward" => horizontal += magnitude,
            "up" => depth -= magnitude,
            "down" => depth += magnitude,
            _ => panic!("Why are we here?"),
        }
    }
    
    println!("{}", depth * horizontal);

}
