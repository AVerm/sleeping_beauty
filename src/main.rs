type Probability = f32;
type Trial = (CoinToss, Day);

fn half(trials: Vec<Trial>) -> Probability {
    let (mut heads, mut total) = (0, 0);
    for trial in trials {
        heads = heads + if trial.0 == CoinToss::HEADS {1} else {0};
        total = total + 1;
    }
    return (heads as Probability) / (total as Probability);
}

fn third(trials: Vec<Trial>) -> Probability {
    let (mut heads, mut total) = (0, 0);
    for trial in trials {
        heads = heads + if trial.0 == CoinToss::HEADS {1} else {0};
        total = total + if trial.0 == CoinToss::TAILS {2} else {1};
    }
    return (heads as Probability) / (total as Probability);
}

fn main() {
    let num_trials = 1000000;
    let trials: Vec<Trial> = (1..=num_trials)
        .map(|_| coin_toss())
        .map(|toss: CoinToss| (toss, select_day(toss)))
        .collect();
    println!("Halfer: {}", half(trials.clone()));
    println!("Thirder: {}", third(trials));
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum CoinToss {
    HEADS,
    TAILS,
}

fn coin_toss() -> CoinToss {
    if rand::random::<bool>() {
        CoinToss::HEADS
    }
    else {
        CoinToss::TAILS
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Day {
    MONDAY,
    TUESDAY,
}

fn select_day(toss: CoinToss) -> Day {
    if toss == CoinToss::HEADS {
        Day::MONDAY
    }
    else {
        if rand::random::<bool>() {
            Day::MONDAY
        }
        else {
            Day::TUESDAY
        }
    }
}
