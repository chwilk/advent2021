use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());
    let contents = fs::read_to_string(filename).expect("File error"); 

    let mut lines = contents.lines();
    let guesses: Vec<u8> = lines.next().unwrap().split(',').filter_map(|x| x.parse::<u8>().ok()).collect();
    let mut boards = Vec::new();

    while lines.next().unwrap_or("eof").len() == 0 {
        let mut card = Bingo {board: [[0; 5];5], marks: [[false; 5];5], won: false};
        for i in 0..5 {
            let row = lines.next().unwrap().split_ascii_whitespace().filter_map(|x| x.parse::<u8>().ok()).collect::<Vec<u8>>();
            println!("{} {}", i, row[0]);
            for j in 0..5 {
                card.board[i][j] = row[j];
            }
        }
        boards.push(card);
    }

    println!("First guess {}", guesses[0]);
    println!("Read in {} boards", boards.len());

    let mut winners = boards.len();

    for guess in 0..guesses.len() {
        for card in 0..boards.len() {
            if !boards[card].won {
                boards[card].mark(guesses[guess]);
                match boards[card].check() {
                    Some(s) => {
                        if winners > 1 {
                            boards[card].won = true;
                            winners -= 1;
                            println!("Removed {}, {} left.", card, winners);
                        } else {
                            println!("Last winning no {}, Board {}, Score {}",
                                     guesses[guess],
                                     card + 1,
                                     s * guesses[guess] as u32);
                            std::process::exit(0);
                        }
                    },
                    None => (),
                }
            }
        }
    }
}

struct Bingo {
    board: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
    won: bool,
}

impl Bingo {
    #[allow(dead_code)]
    fn display(&self) {
        for i in 0..5 {
            println!("{:5}{:5}{:5}{:5}{:5}",
                     self.board[i][0],
                     self.board[i][1],
                     self.board[i][2],
                     self.board[i][3],
                     self.board[i][4],
                     );
            println!("{:5}{:5}{:5}{:5}{:5}",
                     self.marks[i][0],
                     self.marks[i][1],
                     self.marks[i][2],
                     self.marks[i][3],
                     self.marks[i][4],
                     );
        }
    }

    fn mark(&mut self, num: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == num {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn check(&self) -> Option<u32>{
       for i in 0..5 {
           if (self.marks[i][0]
               && self.marks[i][1]
               && self.marks[i][2]
               && self.marks[i][3]
               && self.marks[i][4])
           || (self.marks[0][i]
               && self.marks[1][i]
               && self.marks[2][i]
               && self.marks[3][i]
               && self.marks[4][i]
               ) {
                   println!("Bingo!");
                   return Some(self.score());
               }
       }
       None
    }

    fn score(&self) -> u32 {
        let mut score: u32 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    score += self.board[i][j] as u32;
                }
            }
        }
        score
    }
}
