use std::env;

const PLAY_TO: usize = 21;

// Each 'verse in the multiverse gets a 5-tuple:
// (player1_position, player1_score, player2_position, player2_score, copies)
// where copies is how many of this universe exist.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut p1: usize = 3; // position 4 minus zero index
    let mut p2: usize = 7;
    if args.len() >= 3 { 
        p1 = &args[1].parse::<usize>().unwrap() -1;
        p2 = &args[2].parse::<usize>().unwrap() -1;
    } 


    let mut wins = [0u128, 0u128];

    let mut metaverse = [[[[0u128; PLAY_TO]; PLAY_TO]; 10]; 10];

    // Create initial positions
    metaverse[p1][p2][0][0] = 1;

    let mut player = 0;
    let mut roll = 0; // player's throw (1 of 3 per turn)

    loop { // loop over rolls, alternating players by player var
        // make move
        let mut new_metaverse = [[[[0u128; PLAY_TO]; PLAY_TO]; 10]; 10];
        for p1 in 0..10 {
            for p2 in 0..10 {
                for s1 in 0..PLAY_TO {
                    for s2 in 0..PLAY_TO {
                        if metaverse[p1][p2][s1][s2] > 0 { 
                            for die in 1..=3 { // apply quantum universe splitting
                                // apply the player's roll
                                let mut new_coords = [p1, p2];
                                new_coords[player] = (new_coords[player] + die) % 10;
                                // add starting verse's count to new coords
                                new_metaverse[new_coords[0]][new_coords[1]][s1][s2] +=
                                    metaverse[p1][p2][s1][s2];
                            }
                        }
                    }
                }
            }
        }
        metaverse = new_metaverse;
        print!("Player {}, Roll {}", player + 1, roll);
        if roll == 2 { // After 3 rolls score and switch player
            for p1 in 0..10 {
                for p2 in 0..10 {
                    for s1 in (0..PLAY_TO).rev() {
                        for s2 in (0..PLAY_TO).rev() {
                            if metaverse[p1][p2][s1][s2] > 0 { 
                                // Score players' rounds
                                let coords = [p1, p2];
                                let mut new_scores = [s1,s2];
                                new_scores[player] = new_scores[player] + coords[player] + 1;
                                // Check for wins
                                if new_scores[player] >= PLAY_TO { 
                                    wins[player] += metaverse[p1][p2][s1][s2];
                                } else {
                                    // add starting verse's count to new coord/score points
                                    metaverse[p1][p2][new_scores[0]][new_scores[1]] +=
                                        metaverse[p1][p2][s1][s2];
                                }
                                // decrement starting verse
                                metaverse[p1][p2][s1][s2] = 0;
                            }
                        }
                    }
                }
            }
            // Next player's turn
            roll = 0;
            player = (player + 1) % 2;
        } else { // Handle turn rotation
            roll += 1;
        }

        let verses = count_verses(&metaverse);
        println!(" Univ: {}; p1 wins: {}, p2 wins: {}", verses, wins[0], wins[1]);
        if count_verses(&metaverse) == 0 { break; }
    }
    if wins[0] > wins[1] {
        println!("P1 won most at {} vs P2 {}", wins[0], wins[1]);
    } else {
        println!("P2 won most at {} vs P1 {}", wins[1], wins[0]);
    }
}

fn count_verses(metaverse: &[[[[u128; PLAY_TO]; PLAY_TO]; 10]; 10]) -> u128 {
    let mut sum = 0u128;

    for p1 in 0..10 {
        for p2 in 0..10 {
            for s1 in 0..PLAY_TO {
                for s2 in 0..PLAY_TO {
                    if metaverse[p1][p2][s1][s2] > 0 {
                        sum += metaverse[p1][p2][s1][s2];
                    }
                }
            }
        }
    }
    sum
}

