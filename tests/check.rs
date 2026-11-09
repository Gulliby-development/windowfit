mod common;

use common::row;

#[test]
fn rows() {
    row(&["check"], "", 0, "errors 0 warnings 0\n", "");
    row(&["check"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "errors 0 warnings 0\n", "");
    row(&["check"], "assistant 5 call=c1 | a\n", 0, "message 1: warn: call c1 has no result\nerrors 0 warnings 1\n", "");
    row(&["check"], "tool 5 result=c1 | a\n", 1, "message 1: error: result c1 has no call\nerrors 1 warnings 0\n", "");
    row(&["check"], "tool 5 result=c1 | a\nassistant 5 call=c1 | b\n", 1, "message 1: error: result c1 has no call\nmessage 2: warn: call c1 has no result\nerrors 1 warnings 1\n", "");
    row(&["check"], "assistant 5 call=c1 | a\nassistant 6 call=c1 | b\ntool 7 result=c1 | c\n", 1, "message 2: error: duplicate call c1\nerrors 1 warnings 0\n", "");
    row(&["check"], "assistant 5 call=c1 | a\ntool 7 result=c1 | b\ntool 8 result=c1 | c\n", 1, "message 3: error: second result for call c1\nerrors 1 warnings 0\n", "");
    row(&["check"], "assistant 5 call=c1 | a\nassistant 6 call=c2 | b\ntool 7 result=c2 | c\ntool 8 result=c3 | d\n", 1, "message 1: warn: call c1 has no result\nmessage 4: error: result c3 has no call\nerrors 1 warnings 1\n", "");
    row(&["check"], "assistant 5 call=c1 | a\ntool 7 result=c1 | b\nassistant 6 call=c1 | c\n", 1, "message 3: error: duplicate call c1\nerrors 1 warnings 0\n", "");
    row(&["check"], "user 5 | x\nbot 5 | y\n", 1, "", "windowfit: line 2: bad role \"bot\"\n");
}
