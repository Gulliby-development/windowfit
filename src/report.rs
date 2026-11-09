use crate::{parse, Call, Failure};

/// Lists the conversation's atomic units in message order.
pub fn units(_call: &Call, input: &str) -> Result<(i32, String), Failure> {
    let messages = parse::messages(input)?;
    let units = parse::units(&messages);
    let mut output = String::new();

    for unit in units {
        let first_number = unit.first + 1;
        let mut line = if let Some(second) = unit.second { format!("{first_number}+{} pair {}", second + 1, unit.tokens) } else { format!("{first_number} {} {}", messages[unit.first].role, unit.tokens) };
        if unit.pin {
            line.push_str(" pin");
        }
        line.push('\n');
        output.push_str(&line);
    }

    Ok((0, output))
}

/// Summarizes message, token, pin and unit totals for the conversation.
pub fn report(_call: &Call, input: &str) -> Result<(i32, String), Failure> {
    let messages = parse::messages(input)?;
    let units = parse::units(&messages);
    let mut output = String::new();
    let roles = ["system", "user", "assistant", "tool"];

    for role in roles {
        let count = messages.iter().filter(|message| message.role == role).count();
        let tokens: usize = messages.iter().filter(|message| message.role == role).map(|message| message.tokens).sum();
        output.push_str(&format!("{role} messages={count} tokens={tokens}\n"));
    }

    let total_tokens: usize = messages.iter().map(|message| message.tokens).sum();
    let pinned_tokens: usize = messages.iter().filter(|message| message.pin).map(|message| message.tokens).sum();
    let pair_count = units.iter().filter(|unit| unit.second.is_some()).count();
    output.push_str(&format!("total messages={} tokens={total_tokens}\n", messages.len()));
    output.push_str(&format!("pinned tokens={pinned_tokens}\n"));
    output.push_str(&format!("units {}\n", units.len()));
    output.push_str(&format!("pairs {pair_count}\n"));

    Ok((0, output))
}
