use crate::{check, fit, report, Command};

pub const USAGE: &str = "usage: windowfit fit --budget N [--strategy S] [--reserve R]\n       windowfit units\n       windowfit report\n       windowfit check\n";

pub const COMMANDS: [Command; 4] = [Command { name: "fit", handler: fit::run, values: &["--budget", "--strategy", "--reserve"] }, Command { name: "units", handler: report::units, values: &[] }, Command { name: "report", handler: report::report, values: &[] }, Command { name: "check", handler: check::run, values: &[] }];
