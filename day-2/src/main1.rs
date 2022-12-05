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
    let mut score = 0;
    if plyer.contains("X") {
        score = 1;
    } else if plyer.contains("Y") {
        score = 2;
    } else if plyer.contains("Z") {
        score = 3;
    }

    // score += outcome
    score += outcome(opp, plyer);

    score
}

fn outcome(opp: &str, plyer: &str) -> i32 {
    let plyer_loses = HashMap::from([("X", "B"), ("Y", "C"), ("Z", "A")]);
    let opp_loses = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);

    // A = rock
    // B = paper
    // C = scissors

    // X = rock
    // Y = paper
    // Z = scissors

    // rock beats scissors -  A beats Z, X beats C
    // paper beats rock
    // scissors beats paper

    if opp_loses.get(opp).eq(&Some(&plyer)) {
        return 6;
    }

    if plyer_loses.get(plyer).eq(&Some(&opp)) {
        return 0;
    }
    return 3;
}
