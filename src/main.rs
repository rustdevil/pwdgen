use colored::Colorize;
use rand::prelude::IndexedRandom;
use std::{env, process};

enum PwdOption {
    Lower,
    Upper,
    Numeric,
    All,
}

struct PasswordConfig {
    option: PwdOption,
    length: u32,
}

fn main() {
    let mut args = env::args();
    let args_vec: Vec<String> = env::args().collect(); // Vector so silent mode check doesn't consume arguments

    // Silent mode
    if let Some(arg) = args_vec.get(3) {
        if arg == "s" || arg == "silent" {
            println!("{}", generate_password(get_password_option(&mut args)));

            process::exit(0);
        }
    }

    println!("{} - very bad password generator\n", "pwdgen".blue());

    println!(
        "-> {}: {}",
        "pass".blue(),
        generate_password(get_password_option(&mut args)).yellow()
    )
}

fn get_password_option(args: &mut env::Args) -> PasswordConfig {
    let mut config: PasswordConfig = PasswordConfig {
        option: PwdOption::All,
        length: 16,
    };

    // Get the option
    match args.nth(1) {
        Some(arg) => match arg.as_str() {
            "lower" => config.option = PwdOption::Lower,
            "upper" => config.option = PwdOption::Upper,
            "numeric" => config.option = PwdOption::Numeric,
            "all" => config.option = PwdOption::All,
            &_ => {
                eprintln!("{}", format!("ERROR: {arg} is not a password option").red());
                process::exit(1);
            }
        },
        None => {
            println!(
                "{} \nusage: pwdgen <option> <length>; Options: {}",
                "warn: No options given, defaulting to all.".yellow(),
                "lower, upper, numeric, all\n".blue()
            );
        }
    }

    match args.next() {
        Some(arg) => {
            config.length = match arg.parse() {
                Ok(length) => length,
                Err(_) => {
                    eprintln!(
                        "{}",
                        "ERROR: failed to parse password length, is it a number?".red()
                    );
                    process::exit(1);
                }
            }
        }
        None => {
            println!(
                "{}",
                "warn: No password length specified, defaulting to 16.".yellow()
            );
        }
    }

    config
}

fn generate_password(config: PasswordConfig) -> String {
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
    match config.option {
        PwdOption::Lower => {
            for _n in 1..=config.length {
                password.push(match lowercase.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
        PwdOption::Upper => {
            for _n in 1..=config.length {
                password.push(match uppercase.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
        PwdOption::Numeric => {
            for _n in 1..=config.length {
                password.push(match numeric.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
        PwdOption::All => {
            for _n in 1..=config.length {
                password.push(match all.choose(&mut rng) {
                    Some(char) => *char,
                    None => panic!("bitch idk"),
                });
            }
            password
        }
    }
}
