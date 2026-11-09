use crate::{parse, Call, Failure};

/// Fits a conversation to the requested token limit without splitting atomic units.
pub fn run(call: &Call, input: &str) -> Result<(i32, String), Failure> {
    let budget = call.option("--budget").and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
    let reserve = call.option("--reserve").and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
    let limit = budget.saturating_sub(reserve);
    let strategy = call.option("--strategy").unwrap_or("oldest");

    let messages = parse::messages(input)?;
    let units = parse::units(&messages);
    let pinned_tokens: usize = units.iter().filter(|unit| unit.pin).map(|unit| unit.tokens).sum();
    if pinned_tokens > limit {
        return Err(Failure::new(1, format!("pinned messages need {pinned_tokens} tokens but the limit is {limit}")));
    }

    let mut keep: Vec<bool> = units.iter().map(|unit| unit.pin).collect();
    let mut used;

    match strategy {
        "oldest" => {
            keep.fill(true);
            used = units.iter().map(|unit| unit.tokens).sum();
            while used > limit {
                let Some(index) = keep.iter().enumerate().find_map(|(index, kept)| (*kept && !units[index].pin).then_some(index)) else {
                    break;
                };
                keep[index] = false;
                used -= units[index].tokens;
            }
        }
        "largest" => {
            keep.fill(true);
            used = units.iter().map(|unit| unit.tokens).sum();
            while used > limit {
                let mut candidate: Option<usize> = None;
                for (index, unit) in units.iter().enumerate() {
                    if keep[index] && !unit.pin {
                        let replace = match candidate {
                            None => true,
                            Some(current) => unit.tokens > units[current].tokens || (unit.tokens == units[current].tokens && unit.first < units[current].first),
                        };
                        if replace {
                            candidate = Some(index);
                        }
                    }
                }
                let Some(index) = candidate else {
                    break;
                };
                keep[index] = false;
                used -= units[index].tokens;
            }
        }
        "recent" => {
            used = pinned_tokens;
            for (index, unit) in units.iter().enumerate().rev() {
                if unit.pin {
                    continue;
                }
                if unit.tokens <= limit - used {
                    keep[index] = true;
                    used += unit.tokens;
                } else {
                    break;
                }
            }
        }
        _ => {
            keep.fill(true);
            used = units.iter().map(|unit| unit.tokens).sum();
        }
    }

    let mut message_keep = vec![false; messages.len()];
    for (index, unit) in units.iter().enumerate() {
        if keep[index] {
            message_keep[unit.first] = true;
            if let Some(second) = unit.second {
                message_keep[second] = true;
            }
        }
    }

    let mut output = String::new();
    for (index, message) in messages.iter().enumerate() {
        let decision = if message_keep[index] { "keep" } else { "drop" };
        output.push_str(&format!("{decision} {} {} {}\n", index + 1, message.role, message.tokens));
    }
    output.push_str(&format!("total {used} of {limit}\n"));
    Ok((0, output))
}
