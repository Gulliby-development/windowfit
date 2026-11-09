mod common;

use common::row;

#[test]
fn rows() {
    row(&["report"], "", 0, "system messages=0 tokens=0\nuser messages=0 tokens=0\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=0 tokens=0\npinned tokens=0\nunits 0\npairs 0\n", "");
    row(&["report"], "system 100 pin | You are helpful\nuser 40 | question one\nassistant 30 call=c1 | calling\ntool 200 result=c1 | big output\nuser 20 | question two\nassistant 50 | answer\n", 0, "system messages=1 tokens=100\nuser messages=2 tokens=60\nassistant messages=2 tokens=80\ntool messages=1 tokens=200\ntotal messages=6 tokens=440\npinned tokens=100\nunits 5\npairs 1\n", "");
    row(&["report"], "user 10 pin | a\nuser 30 | b\nuser 20 pin | c\nuser 30 | d\nuser 10 | e\n", 0, "system messages=0 tokens=0\nuser messages=5 tokens=100\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=5 tokens=100\npinned tokens=30\nunits 5\npairs 0\n", "");
    row(&["report"], "tool 7 result=a | x\nassistant 5 call=b | y\n", 0, "system messages=0 tokens=0\nuser messages=0 tokens=0\nassistant messages=1 tokens=5\ntool messages=1 tokens=7\ntotal messages=2 tokens=12\npinned tokens=0\nunits 2\npairs 0\n", "");
    row(&["report"], "assistant 6 call=c1 | a\ntool 8 result=c1 | b\nassistant 7 call=c2 | c\ntool 9 result=c2 | d\n", 0, "system messages=0 tokens=0\nuser messages=0 tokens=0\nassistant messages=2 tokens=13\ntool messages=2 tokens=17\ntotal messages=4 tokens=30\npinned tokens=0\nunits 2\npairs 2\n", "");
    row(&["report"], "system 3 | a\nsystem 4 pin | b\n", 0, "system messages=2 tokens=7\nuser messages=0 tokens=0\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=2 tokens=7\npinned tokens=4\nunits 2\npairs 0\n", "");
}
