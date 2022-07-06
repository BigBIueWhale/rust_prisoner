use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::boxed::Box;
use std::panic;
use std::sync::Mutex;

mod prisoners_riddle;

pub fn calc_stat() -> f64
{
    let mut rng = ChaChaRng::seed_from_u64(0u64);
    let game_settings: prisoners_riddle::GameSettings = prisoners_riddle::GameSettings::new(100, 1000000).unwrap();
    let mut most_recent_progress: Box<f64> = Box::new(-1.0);
    let callback_print_progress = Box::new(move |x: f64| {
        let diff: f64 = (x - *most_recent_progress).abs();
        if diff > 0.01 || (x - 1.0).abs() < 0.000000001 {
            *most_recent_progress = x;
            println!("{0:.5?}%", x * 100.0);
        }
    });
    let game_stats: prisoners_riddle::GameStatistics = prisoners_riddle::collect_game_statistics(&mut rng, game_settings, callback_print_progress);
    game_stats.get_proportion_wins()
}

fn main() {
    let stat: Mutex<f64> = Mutex::new(0.0);
    let result = panic::catch_unwind(|| {
        let res: f64 = calc_stat();
        *stat.lock().unwrap() = res;
    });
    match result {
        Ok(_) => {},
        Err(_) => println!("Caught panic in main"),
    }
    println!("Won {}% of the time", *stat.lock().unwrap() * 100.0);
}

#[cfg(test)]
mod tests {
    use super::calc_stat;
    #[test]
    fn test_prisoners_riddle() {
        let stat = calc_stat();
        let correct_answer = 0.31183;
        let diff: f64 = (stat - correct_answer).abs();
        if diff > 0.001 {
            panic!("Wrong answer: {} Expected answer: {}", stat, correct_answer);
        }
    }
}
