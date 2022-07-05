// Based on: https://youtu.be/iSNsgj1OCLA "The Riddle That Seems Impossible Even If You Know The Answer" Veritasium

use rand::seq::SliceRandom;
use rand_chacha::ChaChaRng;
use std::boxed::Box;

struct Ballot {
    hidden_num: i32,
}

struct Prisoner {
    note_num: i32,
}

fn gen_random_nums(rng: &mut ChaChaRng, num_ballots: i32) -> Vec<i32> {
    assert!(num_ballots > 0);
    let mut nums: Vec<i32> = (1..num_ballots).collect();
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum GameResult {
    Win,
    Lose,
}

pub fn play_game(rng: &mut ChaChaRng, num_ballots: i32) -> GameResult {
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

pub struct GameStatistics {
    proportion_wins: f64,
}

pub struct GameSettings {
    num_ballots: i32,
    num_games: i32,
}

#[derive(Debug)]
pub enum GameSettingsError {
    InvalidNumBallots,
    InvalidNumGames,
}

impl GameSettings {
    pub fn new(num_ballots: i32, num_games: i32) -> Result<GameSettings, GameSettingsError> {
        if num_ballots <= 0 || num_ballots % 2 != 0 {
            return Err(GameSettingsError::InvalidNumBallots);
        }
        if num_games <= 0 {
            return Err(GameSettingsError::InvalidNumGames);
        }
        Ok(GameSettings {
            num_ballots: num_ballots,
            num_games: num_games,
        })
    }
}

#[derive(Debug)]
enum GameStatisticsError {
    InvalidNumGames,
    InvalidNumWins,
    NumWinsGreaterThanNumGames,
}

impl GameStatistics {
    fn new(num_games: i32, num_wins: i32) -> Result<GameStatistics, GameStatisticsError> {
        if num_games <= 0 {
            return Err(GameStatisticsError::InvalidNumGames);
        }
        if num_wins < 0 {
            return Err(GameStatisticsError::InvalidNumWins);
        }
        if num_wins > num_games {
            return Err(GameStatisticsError::NumWinsGreaterThanNumGames);
        }
        Ok(GameStatistics {
            proportion_wins: num_wins as f64 / num_games as f64,
        })
    }
    pub fn get_proportion_wins(&self) -> f64 {
        self.proportion_wins
    }
}

pub fn collect_game_statistics(rng: &mut ChaChaRng, game_settings: GameSettings, mut callback_progress: Box<dyn FnMut(f64) -> ()>) -> GameStatistics {
    let mut num_wins: i32 = 0;
    for game_idx in 0..game_settings.num_games {
        (*callback_progress)(game_idx as f64 / (game_settings.num_games - 1) as f64);
        let res: GameResult = play_game(rng, game_settings.num_ballots);
        match res {
            GameResult::Win => num_wins += 1,
            GameResult::Lose => {},
        }
    }
    (*callback_progress)(1.0);
    GameStatistics::new(game_settings.num_games, num_wins).unwrap()
}
