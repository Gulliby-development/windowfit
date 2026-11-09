mod common;

use common::row;

#[test]
fn rows() {
    row(&["units"], "", 0, "", "");
    row(&["units"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "1 system 100 pin\n2 user 40\n3+4 pair 230\n5 user 20\n6 assistant 50\n", "");
    row(&["units"], "user 5 | a\nassistant 6 call=c1 | b\nuser 7 | c\ntool 8 result=c1 | d\n", 0, "1 user 5\n2+4 pair 14\n3 user 7\n", "");
    row(&["units"], "tool 8 result=c1 | d\nassistant 6 call=c1 | b\n", 0, "1 tool 8\n2 assistant 6\n", "");
    row(&["units"], "assistant 6 call=c1 | a\nassistant 7 call=c1 | b\ntool 8 result=c1 | c\ntool 9 result=c1 | d\n", 0, "1+3 pair 14\n2 assistant 7\n4 tool 9\n", "");
    row(&["units"], "assistant 6 call=c1 | a\nassistant 7 call=c2 | b\ntool 8 result=c2 | c\ntool 9 result=c1 | d\n", 0, "1+4 pair 15\n2+3 pair 15\n", "");
    row(&["units"], "assistant 6 call=c1 pin | a\ntool 8 result=c1 | b\n", 0, "1+2 pair 14 pin\n", "");
    row(&["units"], "assistant 6 call=c1 | a\ntool 8 pin result=c1 | b\n", 0, "1+2 pair 14 pin\n", "");
    row(&["units"], "tool 8 result=c9 | x\n", 0, "1 tool 8\n", "");
    row(&["units"], "system 4 pin | s\nuser 5 pin | u\n", 0, "1 system 4 pin\n2 user 5 pin\n", "");
}
