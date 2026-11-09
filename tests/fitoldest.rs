mod common;

use common::row;

#[test]
fn rows() {
    row(&["fit", "--budget", "1000"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\nkeep 2 user 40\nkeep 3 assistant 30\nkeep 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 440 of 1000\n", "");
    row(&["fit", "--budget", "440"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\nkeep 2 user 40\nkeep 3 assistant 30\nkeep 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 440 of 440\n", "");
    row(&["fit", "--budget", "300"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\ndrop 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 170 of 300\n", "");
    row(&["fit", "--budget", "150"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\ndrop 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\ndrop 5 user 20\nkeep 6 assistant 50\ntotal 150 of 150\n", "");
    row(&["fit", "--budget", "100"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\ndrop 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\ndrop 5 user 20\ndrop 6 assistant 50\ntotal 100 of 100\n", "");
    row(&["fit", "--budget", "300", "--reserve", "100"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "keep 1 system 100\ndrop 2 user 40\ndrop 3 assistant 30\ndrop 4 tool 200\nkeep 5 user 20\nkeep 6 assistant 50\ntotal 170 of 200\n", "");
    row(&["fit", "--budget", "60"], "user 10 | a\nuser 30 | b\nuser 20 | c\nuser 30 | d\nuser 10 | e\n", 0, "drop 1 user 10\ndrop 2 user 30\nkeep 3 user 20\nkeep 4 user 30\nkeep 5 user 10\ntotal 60 of 60\n", "");
    row(&["fit", "--budget", "60", "--strategy", "oldest"], "user 10 pin | a\nuser 30 | b\nuser 20 pin | c\nuser 30 | d\nuser 10 | e\n", 0, "keep 1 user 10\ndrop 2 user 30\nkeep 3 user 20\ndrop 4 user 30\nkeep 5 user 10\ntotal 40 of 60\n", "");
    row(&["fit", "--budget", "35"], "user 10 pin | a\nuser 30 | b\nuser 20 pin | c\nuser 30 | d\nuser 10 | e\n", 0, "keep 1 user 10\ndrop 2 user 30\nkeep 3 user 20\ndrop 4 user 30\ndrop 5 user 10\ntotal 30 of 35\n", "");
    row(&["fit", "--budget", "100"], "", 0, "total 0 of 100\n", "");
}
