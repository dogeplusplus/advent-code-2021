use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let line = line.trim_end();
    let numbers: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let sorted_numbers = quick_sort(numbers);

    let length = sorted_numbers.len();
    let median = match length % 2 == 0 {
        true => (sorted_numbers[length / 2 - 1] + sorted_numbers[length / 2]) / 2,
        false => sorted_numbers[length / 2],
    };

    let cost: i32 = sorted_numbers.iter().map(|x| (x - median).abs()).sum();
    println!("{}", cost);
    Ok(())
}

fn quick_sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    }

    let pivot = list[0];
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in 1..list.len() {
        let elem = list[i];
        if elem <= pivot {
            left.push(elem);
        } else {
            right.push(elem);
        }
    }
    let mut result = quick_sort(left);
    result.append(&mut vec![pivot]);
    result.append(&mut quick_sort(right));

    result
}
