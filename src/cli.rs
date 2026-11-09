use crate::registry::{COMMANDS, USAGE};
use crate::{Call, Failure, Outcome};

enum Parsed {
    Immediate(Outcome),
    Command(&'static crate::Command, Call),
    Invalid(Failure),
}

fn outcome(code: i32, out: String, err: String) -> Outcome {
    Outcome { code, out, err }
}

fn failure(message: impl Into<String>) -> Parsed {
    Parsed::Invalid(Failure::new(2, message))
}

fn decimal_value(value: &str, min: usize, max: usize) -> Option<usize> {
    if value.is_empty() || value.len() > 7 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let number = value.parse::<usize>().ok()?;
    (min..=max).contains(&number).then_some(number)
}

fn parse(args: &[String]) -> Parsed {
    let Some(first) = args.first() else {
        return Parsed::Immediate(outcome(2, String::new(), USAGE.to_string()));
    };

    match first.as_str() {
        "help" | "-h" | "--help" => {
            return Parsed::Immediate(outcome(0, USAGE.to_string(), String::new()));
        }
        "--version" => {
            return Parsed::Immediate(outcome(0, "windowfit 0.1.0\n".to_string(), String::new()));
        }
        _ => {}
    }

    let Some(command) = COMMANDS.iter().find(|command| command.name == first) else {
        return failure(format!("unknown command {first}"));
    };

    let mut call = Call::default();
    let mut index = 1;
    while index < args.len() {
        let word = args[index].as_str();
        if word == "-" || !word.starts_with('-') {
            return failure(format!("unexpected argument {word}"));
        }
        if !command.values.contains(&word) {
            return failure(format!("unknown option {word}"));
        }
        if call.options.iter().any(|(name, _)| name == word) {
            return failure(format!("{word} given twice"));
        }

        let Some(value) = args.get(index + 1) else {
            return failure(match word {
                "--budget" => "--budget needs an integer from 1 to 1000000",
                "--reserve" => "--reserve needs an integer from 0 to 999999",
                "--strategy" => "--strategy needs oldest, largest or recent",
                _ => unreachable!(),
            });
        };
        let valid = match word {
            "--budget" => decimal_value(value, 1, 1_000_000).is_some(),
            "--reserve" => decimal_value(value, 0, 999_999).is_some(),
            "--strategy" => matches!(value.as_str(), "oldest" | "largest" | "recent"),
            _ => false,
        };
        if !valid {
            let message = match word {
                "--budget" => "--budget needs an integer from 1 to 1000000",
                "--reserve" => "--reserve needs an integer from 0 to 999999",
                "--strategy" => "--strategy needs oldest, largest or recent",
                _ => unreachable!(),
            };
            return failure(message);
        }
        call.options.push((word.to_string(), value.clone()));
        index += 2;
    }

    if command.name == "fit" {
        let Some(budget) = call.option("--budget") else {
            return failure("fit needs --budget N");
        };
        let budget = decimal_value(budget, 1, 1_000_000).expect("validated budget");
        let reserve = call.option("--reserve").and_then(|value| decimal_value(value, 0, 999_999)).unwrap_or(0);
        if reserve >= budget {
            return failure("--reserve must be less than --budget");
        }
    }
    Parsed::Command(command, call)
}

/// Runs a command after validating its arguments; invalid invocations never reach a handler.
pub fn run(args: &[String], input: &str) -> Outcome {
    match parse(args) {
        Parsed::Immediate(result) => result,
        Parsed::Invalid(error) => outcome(error.code, String::new(), format!("windowfit: {}\n", error.message)),
        Parsed::Command(command, call) => match (command.handler)(&call, input) {
            Ok((code, text)) => outcome(code, text, String::new()),
            Err(error) => outcome(error.code, String::new(), format!("windowfit: {}\n", error.message)),
        },
    }
}

/// True only when the complete command line is valid and dispatchable.
pub fn reads_stdin(args: &[String]) -> bool {
    matches!(parse(args), Parsed::Command(_, _))
}
