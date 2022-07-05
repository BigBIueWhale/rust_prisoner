// Based on: https://youtu.be/iSNsgj1OCLA "The Riddle That Seems Impossible Even If You Know The Answer" Veritasium

use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaChaRng;

struct Ballot {
    hidden_num: i32,
}

struct Prisoner {
    note_num: i32,
}

fn gen_random_nums(rng: &mut ChaChaRng, num_ballots: i32) -> Vec<i32> {
    assert!(num_ballots > 0);
    let mut nums: Vec<i32> = (0..num_ballots).collect();
    nums.shuffle(rng);
    nums
}

fn gen_ballots(rng: &mut ChaChaRng, num_ballots: i32) -> Vec<Ballot> {
    assert!(num_ballots > 0);
    let hidden = gen_random_nums(rng, num_ballots);
    let mut ballots: Vec<Ballot> = Vec::with_capacity(hidden.len());
    for num in hidden {
        ballots.push(Ballot {
            hidden_num: num,
        });
    }
    ballots
}

enum PrisonerTurnResult {
    StillAlive,
    Dead,
}

fn prisoner_play_turn(ballots: &Vec<Ballot>, prisoner: Prisoner) -> PrisonerTurnResult {
    assert!(ballots.len() > 1);
    assert!(ballots.len() % 2 == 0);
    let num_uncoverings: usize = ballots.len() / 2;
    let first_hidden_num = ballots[prisoner.note_num as usize].hidden_num;
    if first_hidden_num == prisoner.note_num {
        return PrisonerTurnResult::StillAlive;
    }
    let mut trying_next: i32 = first_hidden_num;
    for _ in 0..(num_uncoverings - 1) {
        let hidden_num = ballots[trying_next as usize].hidden_num;
        if hidden_num == prisoner.note_num {
            return PrisonerTurnResult::StillAlive;
        }
        trying_next = hidden_num;
    }
    PrisonerTurnResult::Dead
}

enum GameResult {
    Win,
    Lose,
}

fn play_game(rng: &mut ChaChaRng, num_ballots: i32) -> GameResult {
    assert!(num_ballots > 0);
    let ballots: Vec<Ballot> = gen_ballots(rng, num_ballots);
    let prisoners_nums: Vec<i32> = gen_random_nums(rng, num_ballots);
    let prisoners: Vec<Prisoner> = prisoners_nums
        .iter()
        .map(|&num| Prisoner { note_num: num })
        .collect();
    for prisoner in prisoners {
        let res: PrisonerTurnResult = prisoner_play_turn(&ballots, prisoner);
        match res {
            PrisonerTurnResult::StillAlive => {}
            PrisonerTurnResult::Dead => {
                return GameResult::Lose;
            }
        }
    }
    GameResult::Win
}

fn main() {
    let mut rng = ChaChaRng::seed_from_u64(0u64);
    let num_ballots: i32 = 100;
    let num_games: i32 = 10000000;
    let mut num_wins: i32 = 0;
    for _ in 0..num_games {
        let res: GameResult = play_game(&mut rng, num_ballots);
        match res {
            GameResult::Win => num_wins += 1,
            GameResult::Lose => {},
        }
    }
    let proportion_wins: f64 = num_wins as f64 / num_games as f64;
    println!("Won {}% of the time", proportion_wins * 100.0);
}
