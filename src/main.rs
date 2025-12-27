use std::collections::HashMap;

use clap::{Arg, ArgAction, Command, ValueEnum, value_parser};
use rand::distr::{Distribution, Uniform};

#[derive(Clone, Copy, ValueEnum, Default, Debug, PartialEq)]
pub enum DiceMode {
    #[default]
    None,
    DropHighest,
    DropLowest,
    KeepHighest,
    KeepLowest,
}

fn main() {
    let mut rng = rand::rng();
    let matches = Command::new("Dice Roller")
        .version("2.1.2")
        .about(
            "Rolls dice, provided a count and faces. If none are provided, rolls 1d20 by default",
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("count")
                .value_parser(value_parser!(usize))
                .default_value("1")
                .help("How many times to roll"),
        )
        .arg(
            Arg::new("die")
                .short('d')
                .long("die")
                .value_name("die")
                .value_parser(value_parser!(usize))
                .default_value("20")
                .help("How many die faces"),
        )
        .arg(
            Arg::new("adjust_total")
                .short('a')
                .long("adjust_total")
                .value_name("adjust_total")
                .required(false)
                .value_parser(value_parser!(isize))
                .allow_negative_numbers(true)
                .help("Modifies the Total"),
        )
        .arg(
            Arg::new("roll_mode")
                .long("roll_mode")
                .short('m')
                .value_name("roll_mode")
                .required(false)
                .value_parser(clap::builder::EnumValueParser::<DiceMode>::new())
                .default_value("none")
                .help("What Dice Roll Mode to use"),
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

    let mut count = *matches.get_one("count").unwrap_or(&1);
    if count == 0 {
        println!("Count was 0, setting to 1...");
        count = 1
    };
    let original_count = count;
    let faces = *matches.get_one("die").unwrap_or(&20);
    let extended = *matches.get_one("extended").unwrap_or(&false);
    let timestamp = *matches.get_one("timestamp").unwrap_or(&false);
    let mut dice_mode = *matches.get_one("roll_mode").unwrap_or(&DiceMode::None);
    let adjust_total = *matches.get_one("adjust_total").unwrap_or(&0);
    let die = Uniform::new_inclusive(1, faces).unwrap();
    let mut results = Vec::new();
    if timestamp {
        println!(
            "Timestamp: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f %Z")
        );
    }
    println!("Count: {count}");
    println!("Faces: {faces}");
    println!("Mode: {dice_mode:?}");

    if count <= 1 {
        dice_mode = DiceMode::None
    }

    while count > 0 {
        results.push(die.sample(&mut rng));
        count -= 1;
    }
    let result = match dice_mode {
        DiceMode::DropLowest => remove_lowest_n(&results, 1)
            .iter()
            .sum::<usize>()
            .checked_add_signed(adjust_total)
            .unwrap_or(1),
        DiceMode::DropHighest => remove_highest_n(&results, 1)
            .iter()
            .sum::<usize>()
            .checked_add_signed(adjust_total)
            .unwrap_or(1),
        DiceMode::KeepLowest => results
            .iter()
            .min()
            .unwrap()
            .checked_add_signed(adjust_total)
            .unwrap_or(1),
        DiceMode::KeepHighest => results
            .iter()
            .max()
            .unwrap()
            .checked_add_signed(adjust_total)
            .unwrap_or(1),
        _ => results
            .iter()
            .sum::<usize>()
            .checked_add_signed(adjust_total)
            .unwrap_or(1),
    };

    if dice_mode == DiceMode::KeepHighest {
        println!("Highest: {}", result);
    } else if dice_mode == DiceMode::KeepLowest {
        println!("Lowest: {}", result);
    } else if count > 1 {
        println!("Sum: {}", result);
    } else {
        println!("Result: {}", result);
    }

    println!("Rolls: {results:?}");
    if adjust_total != 0 {
        println!("Adjusted by: {adjust_total:?}")
    }

    if extended && !matches!(dice_mode, DiceMode::KeepHighest | DiceMode::KeepLowest) {
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
    if numbers.len().is_multiple_of(2) {
        let left = numbers[(numbers.len() / 2) - 1];
        let right = numbers[numbers.len() / 2];
        (left + right) / 2
    } else {
        numbers[numbers.len() / 2]
    }
}

fn remove_lowest_n(vec: &[usize], n: usize) -> Vec<usize> {
    if n >= vec.len() {
        return Vec::new();
    }

    let mut sorted_vec = vec.to_owned();
    sorted_vec.sort_unstable();

    vec.iter()
        .filter(|&&x| !sorted_vec[0..n].contains(&x))
        .cloned()
        .collect()
}

fn remove_highest_n(vec: &[usize], n: usize) -> Vec<usize> {
    if n >= vec.len() {
        return Vec::new();
    }

    let mut sorted_vec = vec.to_owned();
    sorted_vec.sort_unstable();

    vec.iter()
        .filter(|&&x| !sorted_vec[vec.len() - n..].contains(&x))
        .cloned()
        .collect()
}
