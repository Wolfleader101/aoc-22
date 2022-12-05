use std::collections::HashMap;
use std::fs;
use std::str;

fn main() {
    // read file input
    let games: i32 = fs::read_to_string("input.txt")
        .expect("Should read file")
        .lines() // each line is a round
        .map(|round: &str| {
            // for each round calculate its score and add to sum
            let (opp, plyer): (&str, &str) = round.split_at(round.find(' ').unwrap());

            player_score(opp.trim(), plyer.trim()) // score = chosen shape
        })
        .sum();

    println!("{games}");
}

fn player_score(opp: &str, plyer: &str) -> i32 {
    // A = rock
    // B = paper
    // C = scissors
    let plyer_loses = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);
    let opp_loses = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let scores = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let mut score: i32 = 0;
    if plyer.eq("X") {
        score += *scores.get(*plyer_loses.get(opp).unwrap()).unwrap();
    } else if plyer.eq("Y") {
        score += *scores.get(opp).unwrap();
        score += 3;
    } else if plyer.eq("Z") {
        score += *scores.get(*opp_loses.get(opp).unwrap()).unwrap();
        score += 6;
    }

    score
}
