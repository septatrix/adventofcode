use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


#[derive(Debug)]
struct BingoField(u64, bool);

#[derive(Debug)]
struct BingoBoard {
    dimensions: usize,
    fields: Vec<BingoField>,
    complete: bool,
}

impl BingoBoard {
    fn check_rows(&self) -> bool {
        for i in 0..self.dimensions {
            if self
                .fields
                .iter()
                .skip(i * self.dimensions)
                .take(self.dimensions)
                .all(|f| f.1)
            {
                return true;
            }
        }
        false
    }
    fn check_columns(&self) -> bool {
        for i in 0..self.dimensions {
            if self
                .fields
                .iter()
                .skip(i)
                .step_by(self.dimensions)
                .all(|f| f.1)
            {
                return true;
            }
        }
        false
    }
    fn check(&self) -> bool {
        self.check_rows() || self.check_columns()
    }
    fn score(&self, called: u64) -> u64 {
        self.fields.iter().filter(|f| !f.1).map(|f| f.0).sum::<u64>() * called
    }
    fn call(&mut self, called: u64) {
        self.fields
            .iter_mut()
            .filter(|f| f.0 == called)
            .for_each(|f| f.1 = true);
    }

    fn parse(board: &str, dimensions: usize) -> Self {
        let fields = board
            .split(char::is_whitespace)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .map(|u| BingoField(u, false))
            .collect::<Vec<_>>();
        BingoBoard { dimensions, fields, complete: false }
    }
}

fn main() {

    let file = File::open("input.txt").expect("ERR: file not found!");
    let mut lines = BufReader::new(file).lines().map(|x| x.unwrap()).peekable();

    let called_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    lines.next();

    let mut boards = Vec::new();
    while lines.peek().is_some() {
        let line_vec = lines
            .by_ref()
            .take_while(|l| !l.trim().is_empty())
            .collect::<Vec<_>>();
        boards.push(BingoBoard::parse(&line_vec.join(" "), line_vec.len()));
    }

    let mut last_score = None;

    for num in called_numbers {
        for board in &mut boards {
            if !board.complete {
                board.call(num);
                if board.check() {
                    let score = board.score(num);
                    if last_score.is_none() {
                        println!("First winning score: {}", score);
                    }
                    last_score = Some(score);
                    board.complete = true;
                }
            }
        }
    }

    println!("Last winning score: {}", last_score.unwrap());
}
