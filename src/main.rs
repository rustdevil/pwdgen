use colored::Colorize;
use rand::prelude::IndexedRandom;
use std::env;
use std::process;

enum PasswordOption {
    Lower,
    Upper,
    Numeric,
    All,
}

fn main() {
    let mut args = env::args();

    println!("{} - very bad password generator\n", "pwdgen".blue());

    println!(
        "-> {}: {}",
        "pass".blue(),
        generate_password(get_password_option(&mut args)).yellow()
    )
}

fn get_password_option(args: &mut env::Args) -> PasswordOption {
    match args.nth(1) {
        Some(arg) => match arg.as_str() {
            "lower" => PasswordOption::Lower,
            "upper" => PasswordOption::Upper,
            "numeric" => PasswordOption::Numeric,
            "all" => PasswordOption::All,
            &_ => {
                eprintln!(
                    "{}",
                    format!("-> ERROR: {arg} is not a password option").red()
                );
                process::exit(1);
            }
        },
        None => {
            println!(
                "{} \nusage: pwdgen <option>; Options: {}",
                "warn: No options given, defaulting to all.".yellow(),
                "lower, upper, numeric, all\n".blue()
            );
            PasswordOption::All
        }
    }
}

fn generate_password(option: PasswordOption) -> String {
    let mut rng = rand::rng();

    let lowercase: Vec<char> = ('a'..='z').collect();
    let uppercase: Vec<char> = ('A'..='Z').collect();
    let numeric: Vec<char> = ('0'..='9').collect();
    let all: Vec<char> = [
        lowercase.clone(),
        uppercase.clone(),
        numeric.clone(),
        vec!['!', '@', '#', '$', '%', '^', '&', '*'],
    ]
    .concat();

    let mut password: String = "".into();
    match option {
        PasswordOption::Lower => {
            for _n in 1..=64 {
                password.push(match lowercase.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
        PasswordOption::Upper => {
            for _n in 1..=64 {
                password.push(match uppercase.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
        PasswordOption::Numeric => {
            for _n in 1..=64 {
                password.push(match numeric.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
        PasswordOption::All => {
            for _n in 1..=64 {
                password.push(match all.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
    }
}
