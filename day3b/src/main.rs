use std::io::{BufRead, BufReader};
use std::fs::File;


fn bit_count(readings: &Vec<Vec<char>>, idx: usize) -> Vec<usize> {
    let ones = readings.iter().filter(|&n| n[idx] == '1').count();
    let zeros = readings.iter().filter(|&n| n[idx] == '0').count();

    vec![zeros,ones]
}


fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot read input"));

    let mut readings = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        readings.push(line.chars().collect());
    }

    let mut oxygen_readings = readings.clone();
    let mut co2_readings = readings.clone();

    let mut index = 0;
    let mut length = oxygen_readings.len();

    while length > 1 {
        let count = bit_count(&oxygen_readings, index);

        let filter_char = if count[1] >= count[0] {
            '1'
        } else {
            '0'
        };

        oxygen_readings.retain(|x| x[index] == filter_char);
        index += 1;
        length = oxygen_readings.len();
    }

    let mut index = 0;
    let mut length = co2_readings.len();

    while length > 1 {
        let count = bit_count(&co2_readings, index);
        let filter_char = if count[0] <= count[1] {
            '0'
        } else {
            '1'
        };
        co2_readings.retain(|x| x[index] == filter_char);
        index += 1;
        length = co2_readings.len();
    }

    let oxygen_rating = isize::from_str_radix(&String::from_iter(oxygen_readings[0].clone()), 2).unwrap();
    let co2_rating = isize::from_str_radix(&String::from_iter(co2_readings[0].clone()), 2).unwrap();

    println!("{}", oxygen_rating * co2_rating);
}
