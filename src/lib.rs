//! windowfit: fit a chat history into a token budget.
pub mod check;
pub mod cli;
pub mod fit;
pub mod parse;
pub mod registry;
pub mod report;

/// An error with its exit status (2 for the command line, 1 for the conversation) and its message.
#[derive(Debug)]
pub struct Failure {
    pub code: i32,
    pub message: String,
}

impl Failure {
    pub fn new(code: i32, message: impl Into<String>) -> Failure {
        Failure { code, message: message.into() }
    }
}

/// What a run gives back: the exit status and the two output strings.
#[derive(Debug, PartialEq)]
pub struct Outcome {
    pub code: i32,
    pub out: String,
    pub err: String,
}

/// One message of the conversation. `tokens` is already settled (a `~` in the input was estimated from the text); `call` and `result` hold the id of a tool call.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: String,
    pub tokens: usize,
    pub pin: bool,
    pub call: Option<String>,
    pub result: Option<String>,
}

/// A unit of the conversation that is kept or dropped as a whole: one message, or a tool call with its result. `first` and `second` are 0-based message positions; `pin` is true when any of its messages is pinned.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub first: usize,
    pub second: Option<usize>,
    pub tokens: usize,
    pub pin: bool,
}

/// The parsed command line of one command: the options given (name and value).
#[derive(Debug, Default)]
pub struct Call {
    pub options: Vec<(String, String)>,
}

impl Call {
    pub fn option(&self, name: &str) -> Option<&str> {
        self.options.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
    }
}

/// A handler returns the exit status and the text for standard output, or a failure.
pub type Handler = fn(&Call, &str) -> Result<(i32, String), Failure>;

/// A row of the command table: the options that take a value.
pub struct Command {
    pub name: &'static str,
    pub handler: Handler,
    pub values: &'static [&'static str],
}

pub fn run(args: &[String], input: &str) -> Outcome {
    cli::run(args, input)
}

/// True when the program has to read standard input for this command line.
pub fn reads_stdin(args: &[String]) -> bool {
    cli::reads_stdin(args)
}
