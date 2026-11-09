mod common;

use common::row;

#[test]
fn rows() {
    row(&["report"], "user ~ | abcd\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=1\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=1\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ | abcde\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=2\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=2\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ |\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=1\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=1\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ |  two spaces\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=3\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=3\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ | caf\u{e9}\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=2\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=2\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ | \u{20ac}\u{20ac}\u{20ac}\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=3\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=3\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ | \u{1f600}\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=1\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=1\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["report"], "user ~ | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=10\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=10\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["units"], "user ~ pin | hello world\nuser 3 | y\n", 0, "1 user 3 pin\n2 user 3\n", "");
}
