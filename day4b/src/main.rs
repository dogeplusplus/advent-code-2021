#[macro_use]
extern crate ndarray;

use std::io::{self, BufReader, BufRead};
use std::fs::File;
use ndarray::{Array2, Axis};

#[derive(Clone,Debug,PartialEq,Eq)]
struct BingoBoard {
    numbers: Array2<u32>,
    marked: Array2<u32>,
}

impl BingoBoard {
    fn new(raw_numbers: Vec<String>) -> Self {
        let numbers: Vec<Vec<u32>> = raw_numbers
            .iter()
            .map(|x| x.split_whitespace().map(|y| y.parse::<u32>().unwrap()).collect())
            .collect();
        let mut matrix = Array2::zeros((numbers.len(), numbers[0].len()));

        for i in 0..numbers.len() {
            for j in 0..numbers[i].len() {
                matrix[[i,j]] = numbers[i][j];
            }
        }

        let dim = matrix.raw_dim();
        Self{
            numbers: matrix,
            marked: Array2::zeros(dim)
        }
    }

    fn mark_number(&mut self, num: u32) {
        for i in 0..self.numbers.len_of(Axis(0)) {
            for j in 0..self.numbers.len_of(Axis(1)) {
                if self.numbers[[i,j]] == num {
                    self.marked[[i,j]] = 1;
                }
            }
        }
    }

    fn bingo(self) -> bool {
        let height = self.marked.len_of(Axis(0));
        let width = self.marked.len_of(Axis(1));

        for i in 0..height {
            if self.marked.slice(s![i,..]).sum() == width as u32 {
                return true;
            }
        }

        for i in 0..width {
            if self.marked.slice(s![..,i]).sum() == height as u32 {
                return true;
            }
        }

       false
    }

    fn score_numbers(self) -> u32 {
        (self.marked.mapv(|x| (x == 0) as u32) * self.numbers).sum()
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
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
    let mut completed_boards = Vec::new();
    for number in numbers {
        for idx in 0..bingo_boards.len() {
            if !completed_boards.contains(&idx) {
                bingo_boards[idx].mark_number(number);
            }
            if bingo_boards[idx].clone().bingo() && !completed_boards.contains(&idx) {
                number_called = number;
                completed_boards.push(idx);
            }

        }
    }

    let index = completed_boards.pop().unwrap();
    let last_board = bingo_boards[index].clone();
    let score = last_board.score_numbers();
    Ok(())
}
