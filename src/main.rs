use std::collections::HashMap;

use clap::{Arg, ArgAction, Command};
use rand::distr::{Distribution, Uniform};

fn main() {
    let mut rng = rand::rng();
    let matches = Command::new("Dice Roller")
        .version("0.1.2")
        .about(
            "Rolls dice, provided a count and faces. If none are provided, rolls 1d20 by default",
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("count")
                .value_parser(clap::value_parser!(usize))
                .default_value("1")
                .help("How many times to roll"),
        )
        .arg(
            Arg::new("faces")
                .short('f')
                .long("faces")
                .value_name("faces")
                .value_parser(clap::value_parser!(usize))
                .default_value("20")
                .help("How many die faces"),
        )
        .arg(
            Arg::new("extended")
                .short('e')
                .long("extended")
                .value_name("extended")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Show extended result set"),
        )
        .arg(
            Arg::new("timestamp")
                .short('t')
                .long("timestamp")
                .value_name("timestamp")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Include Timestamp"),
        )
        .get_matches();

    let mut count: usize = *matches.get_one("count").unwrap_or(&1);
    if count == 0 {
        println!("Count was 0, setting to 1...");
        count = 1
    };
    let original_count = count;
    let faces: usize = *matches.get_one("faces").unwrap_or(&20);
    let extended: bool = *matches.get_one("extended").unwrap_or(&false);
    let timestamp: bool = *matches.get_one("timestamp").unwrap_or(&false);
    let die = Uniform::new_inclusive(1, faces).unwrap();
    let mut results = Vec::new();
    if timestamp {println!("Timestamp: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f %Z"));}
    println!("Count: {count}");
    println!("Faces: {faces}");

    while count > 0 {
        results.push(die.sample(&mut rng));
        count -= 1;
    }
    println!("Sum: {}", results.iter().sum::<usize>());
    println!("Rolls: {results:?}");

    if extended {
        println!("--- Extended Info ---");
        println!("Maximum Possible: {}", original_count * faces);
        println!(
            "Average {original_count}d{faces} Result: {}",
            original_count as f32 * ((faces as f32 / 2.0) + 0.5)
        );
        if extended && original_count > 1 {
            results.sort();
            let average = (results.iter().sum::<usize>() / results.len()) as f32;
            println!("Die Average: {average}");
            let quartile_one =
                median(&results[..(results.len() as f32 / 2.0).floor() as usize]) as f32;
            let quartile_two = median(&results) as f32;
            let quartile_three =
                median(&results[(results.len() as f32 / 2.0).ceil() as usize..]) as f32;
            let mode = mode(&results);
            let qcd = (quartile_three - quartile_one) / (quartile_three + quartile_one);
            let iqr = quartile_three - quartile_one;
            println!("Q1: {quartile_one}");
            println!("Median: {quartile_two}");
            println!("Q3: {quartile_three}");
            println!("Mode: {mode}");
            println!("QCD: {qcd}");
            println!("IQR: {iqr}");
        }
    }
}

fn mode(numbers: &[usize]) -> usize {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Occurrences cannot be 0")
}

fn median(numbers: &[usize]) -> usize {
    if numbers.len() % 2 == 0 {
        let left = numbers[(numbers.len() / 2) - 1];
        let right = numbers[numbers.len() / 2];
        (left + right) / 2
    } else {
        numbers[numbers.len() / 2]
    }
}
