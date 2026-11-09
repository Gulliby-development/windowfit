mod common;

use common::row;

#[test]
fn rows() {
    row(&["fit", "--budget", "60", "--strategy", "largest"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 0, "keep 1 user 10\ndrop 2 user 30\nkeep 3 user 20\ndrop 4 user 30\nkeep 5 user 10\ntotal 40 of 60\n", "");
    row(&["fit", "--budget", "45", "--strategy", "largest"], "user 10 pin | a\nuser 30 | b\nuser 20 pin | c\nuser 30 | d\nuser 10 | e\n", 0, "keep 1 user 10\ndrop 2 user 30\nkeep 3 user 20\ndrop 4 user 30\nkeep 5 user 10\ntotal 40 of 45\n", "");
    row(&["fit", "--budget", "300", "--strategy", "largest"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\nkeep 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 210 of 300\n", "");
    row(&["fit", "--budget", "120", "--strategy", "largest"], "user 30 | a\nuser 50 | b\nuser 50 | c\nuser 20 | d\n", 0, "keep 1 user 30\ndrop 2 user 50\nkeep 3 user 50\nkeep 4 user 20\ntotal 100 of 120\n", "");
    row(&["fit", "--budget", "300", "--strategy", "recent"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\ndrop 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 170 of 300\n", "");
    row(&["fit", "--budget", "400", "--strategy", "recent"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\ndrop 2 user 40\nkeep 3 assistant 30\nkeep 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 400 of 400\n", "");
    row(&["fit", "--budget", "60", "--strategy", "recent"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 0, "drop 1 user 10\ndrop 2 user 30\nkeep 3 user 20\nkeep 4 user 30\nkeep 5 user 10\ntotal 60 of 60\n", "");
    row(&["fit", "--budget", "60", "--strategy", "recent"], "user 10 pin | a\nuser 30 | b\nuser 20 pin | c\nuser 30 | d\nuser 10 | e\n", 0, "keep 1 user 10\ndrop 2 user 30\nkeep 3 user 20\ndrop 4 user 30\nkeep 5 user 10\ntotal 40 of 60\n", "");
    row(&["fit", "--budget", "55", "--strategy", "recent"], "user 10 | a\nuser 40 | b\nuser 5 | c\nuser 20 | d\n", 0, "drop 1 user 10\ndrop 2 user 40\nkeep 3 user 5\nkeep 4 user 20\ntotal 25 of 55\n", "");
    row(&["fit", "--budget", "230", "--strategy", "largest", "--reserve", "30"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\nkeep 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\nkeep 5 user 20\ndrop 6 assistant 50\ntotal 160 of 200\n", "");
}
