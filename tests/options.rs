mod common;

use common::row;

#[test]
fn rows() {
    row(&["fit"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: fit needs --budget N\n");
    row(&["fit", "--budget", "0"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --budget needs an integer from 1 to 1000000\n");
    row(&["fit", "--budget", "1000001"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --budget needs an integer from 1 to 1000000\n");
    row(&["fit", "--budget"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --budget needs an integer from 1 to 1000000\n");
    row(&["fit", "--budget", "100", "--strategy"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --strategy needs oldest, largest or recent\n");
    row(&["fit", "--budget", "100", "--strategy", "latest"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --strategy needs oldest, largest or recent\n");
    row(&["fit", "--budget", "100", "--reserve", "100"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --reserve must be less than --budget\n");
    row(&["fit", "--budget", "100", "--reserve", "x"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --reserve needs an integer from 0 to 999999\n");
    row(&["fit", "--budget", "100", "--reserve", "1000000"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --reserve needs an integer from 0 to 999999\n");
    row(&["fit", "--budget", "100", "--budget", "200"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 2, "", "windowfit: --budget given twice\n");
    row(&["fit", "--reserve", "20", "--budget", "100"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 0, "drop 1 user 10\ndrop 2 user 30\nkeep 3 user 20\nkeep 4 user 30\nkeep 5 user 10\ntotal 60 of 80\n", "");
    row(&["fit", "--budget", "007", "--reserve", "00"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 0, "drop 1 user 10\ndrop 2 user 30\ndrop 3 user 20\ndrop 4 user 30\ndrop 5 user 10\ntotal 0 of 7\n", "");
}
