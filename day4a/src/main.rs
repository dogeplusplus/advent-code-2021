use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Clone,Debug)]
struct BingoBoard {
    numbers: Vec<Vec<u32>>,
    marked: Vec<Vec<u32>>,
}

impl BingoBoard {
    fn new(raw_numbers: Vec<String>) -> Self {
        let numbers: Vec<Vec<u32>> = raw_numbers
            .iter()
            .map(|x| x.split_whitespace().map(|y| y.parse::<u32>().unwrap()).collect())
            .collect();
        let length: usize = numbers.len();
        let height: usize = numbers[0].len();
        Self{
            numbers,
            marked: vec![vec![0; length]; height],
        }
    }

    fn mark_number(&mut self, num: u32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == num {
                    self.marked[i][j] = 1;
                }
            }
        }
    }

    fn bingo(self) -> bool {
        let check_row = self.marked.clone().into_iter().any(|x| x.into_iter().all(|y| y == 1));
        if check_row {
            return true
        }

        for i in 0..self.numbers[0].len() {
            if self.marked.iter().map(|x| x[i]).all(|y| y == 1) {
                return true
            }
        }
       false
    }

    fn score_numbers(self) -> u32 {
        let mut score = 0;
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.marked[i][j] == 0 {
                    score += self.numbers[i][j];
                }
            }
        }
        score
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut f = BufReader::new(f);

    let mut numbers = String::new();
    f.read_line(&mut numbers).expect("Reading numbers failed");
    let numbers = numbers.trim();
    let numbers: Vec<u32> = Vec::from_iter(numbers.split(",").map(|x| x.parse::<u32>().unwrap()));

    let lines: Vec<_> = f.lines().map(|x| x.unwrap()).collect();

    let mut bingo_boards = Vec::new();
    for board in lines.chunks(6) {
        bingo_boards.push(BingoBoard::new(board[1..].to_vec()));
    };

    let mut number_called = 0;
    let mut completed_boards: Vec<BingoBoard> = Vec::new();
    for number in numbers {
        for board in &mut bingo_boards {
            board.mark_number(number);
            if board.clone().bingo() {
                completed_boards.push(board.clone());
            }

        }
        if completed_boards.len() > 0 {
            number_called = number;
            break;
        }
    }

    let mut max_score = 0;
    println!("{:?}", completed_boards);
    for board in completed_boards {
        let score = board.score_numbers();
        if score > max_score {
            max_score = score;
        }
    }
    println!("{}", number_called);
    println!("{}", max_score * number_called);
}
