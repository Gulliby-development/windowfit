use std::collections::HashMap;

use crate::{Failure, Message, Unit};

/// Parses conversation text into messages, reporting the first invalid physical line.
pub fn messages(text: &str) -> Result<Vec<Message>, Failure> {
    let mut parsed = Vec::new();

    for (line_index, raw_line) in text.split('\n').enumerate() {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let trimmed = line.trim_matches([' ', '\t']);
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let line_number = line_index + 1;
        let Some(pipe) = line.find('|') else {
            return Err(line_failure(line_number, "missing \"|\""));
        };
        let head = &line[..pipe];
        let mut fields = ascii_fields(head);
        let role = fields.next().unwrap_or("");
        if !matches!(role, "system" | "user" | "assistant" | "tool") {
            return Err(line_failure(line_number, &format!("bad role \"{role}\"")));
        }

        let token_field = fields.next().unwrap_or("");
        let tokens = if token_field == "~" {
            let text = line[pipe + 1..].strip_prefix(' ').unwrap_or(&line[pipe + 1..]);
            (text.len() / 4 + usize::from(text.len() % 4 != 0)).max(1)
        } else if !token_field.is_empty() && token_field.len() <= 5 && token_field.bytes().all(|byte| byte.is_ascii_digit()) {
            match token_field.parse::<usize>() {
                Ok(value) if value > 0 => value,
                _ => return Err(line_failure(line_number, &format!("bad tokens \"{token_field}\""))),
            }
        } else {
            return Err(line_failure(line_number, &format!("bad tokens \"{token_field}\"")));
        };

        let mut pin = false;
        let mut call = None;
        let mut result = None;
        for flag in fields {
            enum FlagKind {
                Pin,
                Call,
                Result,
            }

            let (kind, id) = if flag == "pin" {
                (FlagKind::Pin, None)
            } else if let Some(id) = flag.strip_prefix("call=") {
                if !valid_id(id) {
                    return Err(line_failure(line_number, &format!("bad flag \"{flag}\"")));
                }
                (FlagKind::Call, Some(id))
            } else if let Some(id) = flag.strip_prefix("result=") {
                if !valid_id(id) {
                    return Err(line_failure(line_number, &format!("bad flag \"{flag}\"")));
                }
                (FlagKind::Result, Some(id))
            } else {
                return Err(line_failure(line_number, &format!("bad flag \"{flag}\"")));
            };

            match kind {
                FlagKind::Pin => {
                    if pin {
                        return Err(line_failure(line_number, &format!("duplicate flag \"{flag}\"")));
                    }
                    pin = true;
                }
                FlagKind::Call => {
                    if call.is_some() {
                        return Err(line_failure(line_number, &format!("duplicate flag \"{flag}\"")));
                    }
                    call = id.map(str::to_owned);
                }
                FlagKind::Result => {
                    if result.is_some() {
                        return Err(line_failure(line_number, &format!("duplicate flag \"{flag}\"")));
                    }
                    result = id.map(str::to_owned);
                }
            }
        }

        if call.is_some() && role != "assistant" {
            return Err(line_failure(line_number, "call needs role assistant"));
        }
        if result.is_some() && role != "tool" {
            return Err(line_failure(line_number, "result needs role tool"));
        }

        parsed.push(Message { role: role.to_owned(), tokens, pin, call, result });
    }

    Ok(parsed)
}

fn line_failure(line_number: usize, reason: &str) -> Failure {
    Failure::new(1, format!("line {line_number}: {reason}"))
}

fn ascii_fields(head: &str) -> impl Iterator<Item = &str> {
    head.split([' ', '\t']).filter(|field| !field.is_empty())
}

fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 12 && id.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Returns, for each message, the earlier call position answered by that message, if any.
pub fn answers(messages: &[Message]) -> Vec<Option<usize>> {
    let mut first_calls = HashMap::new();
    let mut answered = vec![false; messages.len()];
    let mut result = vec![None; messages.len()];

    for (index, message) in messages.iter().enumerate() {
        if let Some(id) = &message.call {
            first_calls.entry(id.as_str()).or_insert(index);
        }
        if let Some(id) = &message.result {
            if let Some(&call_index) = first_calls.get(id.as_str()) {
                if call_index < index && !answered[call_index] {
                    result[index] = Some(call_index);
                    answered[call_index] = true;
                }
            }
        }
    }

    result
}

/// Groups answered calls with their first eligible result and leaves all other messages singleton.
pub fn units(messages: &[Message]) -> Vec<Unit> {
    let answered = answers(messages);
    let mut results = vec![None; messages.len()];
    let mut result_positions = vec![false; messages.len()];
    for (result_index, call_index) in answered.into_iter().enumerate() {
        if let Some(call_index) = call_index {
            results[call_index] = Some(result_index);
            result_positions[result_index] = true;
        }
    }

    let mut units = Vec::new();
    for (index, message) in messages.iter().enumerate() {
        if result_positions[index] {
            continue;
        }
        let second = results[index];
        let (tokens, pin) = if let Some(second_index) = second {
            let result = &messages[second_index];
            (message.tokens + result.tokens, message.pin || result.pin)
        } else {
            (message.tokens, message.pin)
        };
        units.push(Unit { first: index, second, tokens, pin });
    }

    units
}
