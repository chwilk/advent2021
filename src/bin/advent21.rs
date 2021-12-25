use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut p1 = &args[1].parse::<i32>().unwrap() -1;
    let mut p2 = &args[2].parse::<i32>().unwrap() -1;
    let mut p1score = 0;
    let mut p2score = 0;

    let mut die = Die { roll: 0, rolls: 0};
    loop {
        p1 += die.roll();
        p1 %= 10;
        p1score += p1 + 1;
        if p1score >= 1000 {
            println!("Reached roll {} X loser {} = product {}", die.rolls, p2score, die.rolls*p2score);
            break;
        }
        println!("Player 1 on space {} with score {}", p1, p1score);
        p2 += die.roll();
        p2 %= 10;
        p2score += p2 + 1;
        if p2score >= 1000 {
            println!("Reached roll {} X loser {} = product {}", die.rolls, p1score, die.rolls*p1score);
            break;
        }
        println!("Player 2 on space {} with score {}", p2, p2score);
    }
}

struct Die {
    roll: i32,
    rolls: i32,
}

impl Die {
    fn roll(&mut self) -> i32 {
        let mut this_roll = 0;
        self.rolls += 3;
        for _i in 0..3 {
            self.roll += 1;
            self.roll %= 100;
            this_roll += self.roll;
        }
        this_roll
    }
}
