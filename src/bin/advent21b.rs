use std::env;

// Each 'verse in the multiverse gets a 5-tuple:
// (player1_position, player1_score, player2_position, player2_score, copies)
// where copies is how many of this universe exist.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut p1 = 3; // position 4 minus zero index
    let mut p2 = 7;
    if args.len() >= 3 { 
        p1 = &args[1].parse::<i32>().unwrap() -1;
        p2 = &args[2].parse::<i32>().unwrap() -1;
    } 

    let play_to = 21;

    let mut wins = [0u128,0u128];

    let mut multiverse = Vec::new();

    multiverse.push((p1, 0, p2, 0, 1u128));
    let mut player = 0;
    let mut roll = 0; // player's throw (1 of 3 per turn)

    loop { // loop over rolls, alternating players by player var
        let mut new_verses = Vec::new();
        for i in 0..multiverse.len() {
            if multiverse[i].4 < 1 { continue; } // don't talk to uninstantiated verses.
            for schism in 1..=3 { // apply quantum universe splitting
                // copy the verse
                //let mut newverse = (verse.0, verse.1, verse.2, verse.3, verse.4);
                let mut newverse = multiverse[i];
                // score the player's roll
                if player == 0 {
                    newverse.0 = (newverse.0 + schism) % 10;
                } else {
                    newverse.2 = (newverse.2 + schism) % 10;
                }
                
                let mut found = false;
                for j in 0..multiverse.len() { // does this new verse (copy) exist in multiverse?
                    if multiverse[j].0 == newverse.0 &&
                       multiverse[j].1 == newverse.1 &&
                       multiverse[j].2 == newverse.2 &&
                       multiverse[j].3 == newverse.3 {
                           //     yes: increment that verse's copies
                           multiverse[j].4 += newverse.4;
                           found = true;
                       }
                }
                if !found { //     no: push copy
                    new_verses.push(newverse)
                }
            }
            //delete old verse;
            multiverse[i].4 = 0;
        }
        // append in new verses
        println!("player {}, roll {}: univ {} new verses {}", player + 1, roll, multiverse.len(), new_verses.len());
        multiverse.append(&mut new_verses);
        roll += 1;
        if roll > 2 { // After 3 rolls score and switch player
            for i in 0..multiverse.len() { // are there any winners?
                if multiverse[i].4 == 0 { continue; }
                if player == 0 {
                    multiverse[i].1 += multiverse[i].0;
                } else {
                    multiverse[i].3 += multiverse[i].2;
                }
                if multiverse[i].1 >= play_to { // player1 wins these verses
                    wins[0] += multiverse[i].4;
                    multiverse[i].4 = 0; // delete the winners
                } else if multiverse[i].3 >= play_to { // player2 wins these verses
                    wins[1] += multiverse[i].4;
                    multiverse[i].4 = 0; // delete the winners
                }
            }
            // Next player's turn
            roll = 0;
            player = (player + 1) % 2;
            println!("p1 wins: {}, p2 wins: {}", wins[0], wins[1]);
        }
        // clean up zeroed verses
        multiverse = multiverse.into_iter().filter(|v| v.4 > 0).collect();
        if multiverse.len() == 0 { break; }
    }
    if wins[0] > wins[1] {
        println!("P1 won most at {} vs P2 {}", wins[0], wins[1]);
    } else {
        println!("P2 won most at {} vs P1 {}", wins[1], wins[0]);
    }
}

