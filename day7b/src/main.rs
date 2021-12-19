use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let line = line.trim_end();
    let numbers: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let sorted_numbers = quick_sort(numbers);

    let length = sorted_numbers.len() as i64;

    let sum_squares: i64 = sorted_numbers.iter().map(|x| x*x).sum();
    let linear_term: i64 = sorted_numbers.iter().sum();

    let max_num = sorted_numbers.iter().max().expect("should have max");
    let mut lowest_cost = 1e100 as i64;
    for i in 0..=*max_num {
        let cost = (length * i * i  - 2 * linear_term * i + sum_squares + median_cost(i, &sorted_numbers)) / 2;
        if cost < lowest_cost {
            lowest_cost = cost;
        }
    }

    println!("{}", lowest_cost);
    Ok(())
}

fn median_cost(n: i64, numbers: &Vec<i64>) -> i64 {
    numbers
        .iter()
        .map(|x| (x - n).abs())
        .sum()
}

fn quick_sort(list: Vec<i64>) -> Vec<i64> {
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
