use colored::Colorize;
use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

fn main() {
    run_game();
}

fn run_game() {
    let rng = rand::thread_rng();
    let regenerated_in_a_row = 0;
    let already_shown: Vec<(i32, i32)> = vec![];
    let mut answered = 0;

    let mut times: Vec<f64> = vec![];
    let mut correct_answers = 0;

    game_loop(
        AddGame,
        regenerated_in_a_row,
        already_shown,
        &mut answered,
        rng,
        &mut times,
        &mut correct_answers,
    );

    // Remove fastest 5 and slowest 5
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let normalized_times: Vec<f64> = times
        .iter()
        .skip(5)
        .take(times.len() - 10)
        .cloned()
        .collect();

    let avg_time = if !normalized_times.is_empty() {
        normalized_times.iter().sum::<f64>() / normalized_times.len() as f64
    } else {
        0.0
    };

    println!("{}", "Summary:".bold().underline());
    println!(
        "Total questions answered: {}\nCorrect answers: {}\nAccuracy: {:.2}%",
        answered,
        correct_answers,
        (correct_answers as f64 / answered as f64) * 100.0
    );
    println!(
        "Average time (excluding fastest 5 and slowest 5): {:.2} seconds",
        avg_time
    );
}

fn game_loop<GameType: Game>(
    _game_type: GameType,
    mut regenerated_in_a_row: i32,
    mut already_shown: Vec<(i32, i32)>,
    answered: &mut i32,
    mut rng: rand::prelude::ThreadRng,
    times: &mut Vec<f64>,
    correct_answers: &mut i32,
) {
    loop {
        if regenerated_in_a_row >= 50 {
            already_shown = vec![];
        }

        if *answered >= 50 {
            break;
        } else {
            *answered += 1;
        }

        let game_res = GameType::values(&mut rng);

        if already_shown.contains(&(game_res.val_1, game_res.val_2)) {
            regenerated_in_a_row += 1;
            continue;
        }

        already_shown.push((game_res.val_1, game_res.val_2));

        let start_time = Instant::now();
        let response: i32 = user_input_loop(game_res.val_1, game_res.val_2);

        let elapsed = start_time.elapsed().as_secs_f64();
        times.push(elapsed);

        if response == game_res.result {
            println!("{}", "correct!".green().bold());
            *correct_answers += 1;
        } else {
            println!(
                "{} {}",
                "incorrect!".red().bold(),
                format!("correct: {}", game_res.result).yellow()
            );
        }
        println!();
    }
}

/// does not handle the correct case, thats done later
fn user_input_loop(val_1: i32, val_2: i32) -> i32 {
    loop {
        print!("{}", format!("{val_1} + {val_2} = ").blue().bold());
        stdout().flush().unwrap();

        match get_input().trim().parse() {
            Ok(v) => return v,
            Err(_) => {
                print!("{}", format!("{val_1} + {val_2} = ").blue().bold());
                stdout().flush().unwrap();

                eprintln!("{}", "not a number".red().bold());
                println!();
                continue;
            }
        }
    }
}

fn get_input() -> String {
    let mut buf = Default::default();
    stdin().read_line(&mut buf).expect("failed reading line");
    buf
}

struct AddGame;

impl Game for AddGame {
    fn values(rng: &mut rand::prelude::ThreadRng) -> GameRes {
        let val_1 = rng.gen_range(1..=90);
        let val_2 = rng.gen_range(1..=9);
        let result = val_1 + val_2;

        GameRes {
            val_1,
            val_2,
            result,
        }
    }
}

trait Game {
    fn values(rng: &mut rand::prelude::ThreadRng) -> GameRes;
}

struct GameRes {
    val_1: i32,
    val_2: i32,
    result: i32,
}
