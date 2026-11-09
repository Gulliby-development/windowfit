<p align="center"><img src="docs/logo.svg" width="96" alt="" /></p>

# windowfit

[![CI](https://github.com/Gulliby-development/windowfit/actions/workflows/ci.yml/badge.svg)](https://github.com/Gulliby-development/windowfit/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

windowfit is a Rust 2021 library and command-line tool for fitting a plain-text chat history into a token budget. It keeps assistant tool calls together with their matching tool results and never drops pinned messages. It is intended for people building chat clients, agents, and evaluation harnesses, and can be used interactively, in scripts, or in CI. The project uses only the Rust standard library. It is designed for stable Rust on Windows, macOS, and Linux; no executions on those platforms are claimed here.

## Build and run

Build the binary and library with:

```sh
cargo build
```

For example, fit a history from a file:

```sh
cargo run -- fit --budget 8000 < chat.txt
```

All commands read the conversation from standard input. The command-line interface is:

```text
usage: windowfit fit --budget N [--strategy S] [--reserve R]
       windowfit units
       windowfit report
       windowfit check
```

The default fit strategy is `oldest`, and the default reserve is zero. `--budget` accepts 1 through 1,000,000; `--reserve` accepts 0 through 999,999 and must be less than the budget. Both accept one to seven decimal digits, with leading zeros allowed. `--strategy` accepts exactly `oldest`, `largest`, or `recent`; options can be supplied in any order. A reserve reduces the usable limit: limit = budget - reserve.

The first word may be `help`, `-h`, or `--help` to print the synopsis, or `--version` to print `windowfit 0.1.0`. Both help and version ignore following words and exit with status 0. With no word, the synopsis is written to standard error and the program exits with status 2. An unrecognized first word fails as `windowfit: unknown command WORD`. Commands do not accept positional arguments; options are accepted only by `fit`.

Arguments are validated from left to right. A non-option word (including `-`) fails as `windowfit: unexpected argument WORD`; an unsupported option as `windowfit: unknown option WORD`; and a repeated option as `windowfit: WORD given twice`. Each option consumes the next word as its value, even if that value looks like an option. Missing or invalid values fail immediately with one of these exact messages:

- `windowfit: --budget needs an integer from 1 to 1000000`
- `windowfit: --reserve needs an integer from 0 to 999999`
- `windowfit: --strategy needs oldest, largest or recent`

After the words have been read, `fit` without a budget fails as `windowfit: fit needs --budget N`; a reserve greater than or equal to the budget fails as `windowfit: --reserve must be less than --budget`. Command-line failures exit 2, write no standard output, and write `windowfit: MESSAGE` followed by LF to standard error. Invalid command lines do not read standard input. If reading standard input fails, the binary writes `windowfit: cannot read standard input` to standard error and exits 1. The library's `reads_stdin` reports true only for a valid command invocation that will dispatch to a command.

## Conversation format

Input is UTF-8 text, split into physical lines at LF. One CR immediately before LF, or at the end of the input, is removed. Empty lines, lines containing only spaces or tabs, and lines whose first non-space/tab character is `#` are ignored. Physical line numbers still include ignored lines; message numbers count only the remaining lines, starting at one.

Each message is written as:

```text
ROLE TOKENS [FLAG]... | TEXT
```

The head is the text before the first pipe. Runs of spaces and tabs separate its fields; no other whitespace does. The text begins after the first pipe, with at most one leading space removed. It can be empty and can contain further pipes.

- `ROLE` is one of `system`, `user`, `assistant`, or `tool`.
- `TOKENS` is a positive one-to-five-digit decimal number, or `~`. The estimate uses the UTF-8 byte length of the text, divided by four and rounded up, with a minimum of one. This is a byte-based estimate, not model tokenization.
- `pin` marks a message that must be kept.
- `call=ID` marks an assistant tool call; `result=ID` marks a tool result. IDs contain 1 to 12 ASCII letters, digits, or underscores. Each flag kind can appear at most once per message.

A malformed conversation fails at the first offending physical line. Validation proceeds in this order: a missing pipe; an invalid role; missing or invalid token field; flags from left to right (invalid or unknown flag, then duplicate flag); and finally call/result role compatibility. The exact parser messages are:

- `windowfit: line N: missing "|"`
- `windowfit: line N: bad role "ROLE"`
- `windowfit: line N: bad tokens "FIELD"`
- `windowfit: line N: bad flag "FLAG"`
- `windowfit: line N: duplicate flag "FLAG"`
- `windowfit: line N: call needs role assistant`
- `windowfit: line N: result needs role tool`

The quoted placeholders above are replaced by the physical line number and the offending field as written. An empty conversation is valid.

## Atomic units and fitting

The first message with a given call ID is that ID's call. The first later result with the same ID answers it, if it has not already been answered. A paired call and result form one unit; every other message is a one-message unit, including unanswered calls and results with no eligible call. Units are ordered by their first message. A unit's cost is the sum of its messages' token counts, and a unit is pinned if either message is pinned.

Fit computes the limit after reserve, then rejects the conversation if the total cost of pinned units exceeds that limit. The error is `windowfit: pinned messages need P tokens but the limit is LIMIT`. Otherwise it keeps or drops complete units only, never dropping a pinned unit, and prints a decision for every message in original order. With no messages, fit prints only `total 0 of LIMIT`.

- `oldest` repeatedly drops the kept, unpinned unit whose first message is earliest until the total fits.
- `largest` repeatedly drops the kept, unpinned unit with the greatest cost; ties go to the earliest first message.
- `recent` starts with pinned units, then considers unpinned units from newest to oldest. It keeps each while it fits alongside the units already kept. At the first unit that does not fit, it stops; that unit and all older unpinned units are dropped, even if an older unit would fit by itself.

Fit prints `keep N ROLE TOKENS` or `drop N ROLE TOKENS` for each message, followed by `total USED of LIMIT`. The message's own token count appears on each decision line. A pinned pair's combined cost is used for feasibility and selection, while output still lists both messages separately.

## Other commands and output

- `units` prints one line per unit: `N ROLE TOKENS` for a singleton, or `N+M pair TOKENS` for a pair. It appends ` pin` when any message in that unit is pinned. Empty input prints no unit lines.
- `report` always prints these eight lines, in order: `system messages=M tokens=T`, `user messages=M tokens=T`, `assistant messages=M tokens=T`, `tool messages=M tokens=T`, `total messages=M tokens=T`, `pinned tokens=T`, `units N`, and `pairs N`. `M` is the message count for that role (or the total count on the total line); `T` is its token sum. Pinned tokens sum only pinned messages, not the combined costs of pinned units. On empty input, every count and token total is zero, producing `system messages=0 tokens=0`, `user messages=0 tokens=0`, `assistant messages=0 tokens=0`, `tool messages=0 tokens=0`, `total messages=0 tokens=0`, `pinned tokens=0`, `units 0`, and `pairs 0`.
- `check` reports diagnostics in message order, followed by `errors E warnings W`. Its exact diagnostic templates are `message N: error: duplicate call ID`, `message N: error: result ID has no call`, `message N: error: second result for call ID`, and `message N: warn: call ID has no result`. `N` is the message number and `ID` is the call/result identifier. A clean or empty conversation prints `errors 0 warnings 0`. Check returns status 1 if there are errors and still writes its report to standard output.

Successful command output goes to standard output, with lines ending in LF; standard error is empty. Errors are printed as one line, `windowfit: MESSAGE`, to standard error, with no standard output. Command-line errors exit 2, conversation errors exit 1. The exception is `check`, which uses status 1 for diagnostics while retaining its report on standard output. The binary decodes standard input with lossy UTF-8 conversion, so malformed byte sequences are replaced rather than rejected; parser validation applies to the resulting text.

### Examples

The following are POSIX shell examples run from the repository directory. Their commands and output are shown exactly:

```text
$ printf 'system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n' | cargo run --quiet -- fit --budget 300
keep 1 system 100
drop 2 user 40
drop 3 assistant 30
drop 4 tool 200
keep 5 user 20
keep 6 assistant 50
total 170 of 300
$ printf 'system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n' | cargo run --quiet -- units
1 system 100 pin
2 user 40
3+4 pair 230
5 user 20
6 assistant 50
$ printf 'system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n' | cargo run --quiet -- fit --budget 400 --strategy recent --reserve 20
keep 1 system 100
drop 2 user 40
drop 3 assistant 30
drop 4 tool 200
keep 5 user 20
keep 6 assistant 50
total 170 of 380
$ printf 'assistant 20 call=a1 | one\ntool ~ result=a2 | data here\n' | cargo run --quiet -- check; echo "exit $?"
message 1: warn: call a1 has no result
message 2: error: result a2 has no call
errors 1 warnings 1
exit 1
```

## Development

Run the formatter, check formatting, lint all targets with warnings denied, and run the tests:

```sh
cargo fmt
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The MIT license is in [LICENSE](LICENSE).

---

Built with [BAKED](https://github.com/Gulliby-development/baked), an open-source coding agent (bench round R90).
