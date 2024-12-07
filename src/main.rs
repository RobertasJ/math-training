use colored::Colorize;
use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let mut regenerated_in_a_row = 0;
    let mut already_shown: Vec<(i32, i32)> = vec![];
    let mut answered = 0;

    let mut times: Vec<f64> = vec![];
    let mut correct_answers = 0;

    loop {
        if regenerated_in_a_row >= 50 {
            already_shown = vec![];
        }

        if answered >= 50 {
            break;
        } else {
            answered += 1;
        }

        let val_1 = rng.gen_range(1..=90);
        let val_2 = rng.gen_range(1..=9);

        if already_shown.contains(&(val_1, val_2)) {
            regenerated_in_a_row += 1;
            continue;
        }

        already_shown.push((val_1, val_2));

        let result = val_1 + val_2;

        let start_time = Instant::now();
        let response: i32 = {
            let res;
            loop {
                print!("{}", format!("{val_1} + {val_2} = ").blue().bold());
                stdout().flush().unwrap();

                match get_input().trim().parse() {
                    Ok(v) => {
                        res = v;
                        break;
                    }
                    Err(_) => {
                        print!("{}", format!("{val_1} + {val_2} = ").blue().bold());
                        stdout().flush().unwrap();

                        eprintln!("{}", "not a number".red().bold());
                        println!();
                        continue;
                    }
                }
            }
            res
        };

        let elapsed = start_time.elapsed().as_secs_f64();
        times.push(elapsed);

        if response == result {
            println!("{}", "correct!".green().bold());
            correct_answers += 1;
        } else {
            println!(
                "{} {}",
                "incorrect!".red().bold(),
                format!("correct: {result}").yellow()
            );
        }
        println!();
    }

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
        ((correct_answers as f64 - 10.0) / answered as f64) * 100.0
    );
    println!(
        "Average time (excluding fastest 5 and slowest 5): {:.2} seconds",
        avg_time
    );
}

fn get_input() -> String {
    let mut buf = Default::default();
    stdin().read_line(&mut buf).expect("failed reading line");
    buf
}
