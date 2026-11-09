use std::collections::HashMap;

use crate::{parse, Call, Failure};

/// Reports calls and results that do not form a valid interaction.
pub fn run(_call: &Call, input: &str) -> Result<(i32, String), Failure> {
    let messages = parse::messages(input)?;
    let answers = parse::answers(&messages);
    let resolved_results: Vec<bool> = answers.iter().map(Option::is_some).collect();
    let mut answered_calls = vec![false; messages.len()];
    for call_index in answers.into_iter().flatten() {
        answered_calls[call_index] = true;
    }

    let mut first_calls = HashMap::new();
    let mut diagnostics: Vec<Option<(&str, String)>> = vec![None; messages.len()];
    let mut errors = 0;
    let mut warnings = 0;

    for (index, message) in messages.iter().enumerate() {
        if let Some(id) = &message.call {
            if first_calls.contains_key(id.as_str()) {
                diagnostics[index] = Some(("error", format!("duplicate call {id}")));
                errors += 1;
            } else {
                first_calls.insert(id.as_str(), index);
            }
        }

        if let Some(id) = &message.result {
            match first_calls.get(id.as_str()) {
                None => {
                    diagnostics[index] = Some(("error", format!("result {id} has no call")));
                    errors += 1;
                }
                Some(_) if resolved_results[index] => {}
                Some(_) => {
                    diagnostics[index] = Some(("error", format!("second result for call {id}")));
                    errors += 1;
                }
            }
        }
    }

    for (id, &call_index) in &first_calls {
        if !answered_calls[call_index] {
            diagnostics[call_index] = Some(("warn", format!("call {id} has no result")));
            warnings += 1;
        }
    }

    let mut output = String::new();
    for (index, diagnostic) in diagnostics.into_iter().enumerate() {
        if let Some((level, problem)) = diagnostic {
            output.push_str(&format!("message {}: {level}: {problem}\n", index + 1));
        }
    }
    output.push_str(&format!("errors {errors} warnings {warnings}\n"));
    Ok((i32::from(errors > 0), output))
}
