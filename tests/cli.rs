mod common;

use common::row;

#[test]
fn rows() {
    row(&[], "", 2, "", "usage: windowfit fit --budget N [--strategy S] [--reserve R]\n       windowfit units\n       windowfit report\n       windowfit check\n");
    row(&["help"], "", 0, "usage: windowfit fit --budget N [--strategy S] [--reserve R]\n       windowfit units\n       windowfit report\n       windowfit check\n", "");
    row(&["-h", "x"], "", 0, "usage: windowfit fit --budget N [--strategy S] [--reserve R]\n       windowfit units\n       windowfit report\n       windowfit check\n", "");
    row(&["--help"], "", 0, "usage: windowfit fit --budget N [--strategy S] [--reserve R]\n       windowfit units\n       windowfit report\n       windowfit check\n", "");
    row(&["--version"], "", 0, "windowfit 0.1.0\n", "");
    row(&["--version", "x"], "", 0, "windowfit 0.1.0\n", "");
    row(&["frob"], "", 2, "", "windowfit: unknown command frob\n");
    row(&[""], "", 2, "", "windowfit: unknown command \n");
    row(&["Fit"], "", 2, "", "windowfit: unknown command Fit\n");
    row(&["units", "extra"], "", 2, "", "windowfit: unexpected argument extra\n");
    row(&["report", "-"], "", 2, "", "windowfit: unexpected argument -\n");
    row(&["check", "--bogus"], "", 2, "", "windowfit: unknown option --bogus\n");
    row(&["units", "--budget", "5"], "", 2, "", "windowfit: unknown option --budget\n");
}
