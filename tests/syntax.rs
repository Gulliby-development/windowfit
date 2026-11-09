mod common;

use common::row;

#[test]
fn rows() {
    row(&["report"], "", 0, "system messages=0 tokens=0\nuser messages=0 tokens=0\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=0 tokens=0\npinned tokens=0\nunits 0\npairs 0\n", "");
    row(&["report"], "\n# note\n   \nuser 5 | x\n", 0, "system messages=0 tokens=0\nuser messages=1 tokens=5\nassistant messages=0 tokens=0\ntool messages=0 tokens=0\ntotal messages=1 tokens=5\npinned tokens=0\nunits 1\npairs 0\n", "");
    row(&["units"], "  user\t 5 pin  |x", 0, "1 user 5 pin\n", "");
    row(&["units"], "user 5\n", 1, "", "windowfit: line 1: missing \"|\"\n");
    row(&["units"], "bot 5 | x\n", 1, "", "windowfit: line 1: bad role \"bot\"\n");
    row(&["units"], "| x\n", 1, "", "windowfit: line 1: bad role \"\"\n");
    row(&["units"], "User 5 | x\n", 1, "", "windowfit: line 1: bad role \"User\"\n");
    row(&["units"], "user | x\n", 1, "", "windowfit: line 1: bad tokens \"\"\n");
    row(&["units"], "user 100000 | x\n", 1, "", "windowfit: line 1: bad tokens \"100000\"\n");
    row(&["units"], "user 99999 | x\n", 0, "1 user 99999\n", "");
    row(&["units"], "user 5 pins | x\n", 1, "", "windowfit: line 1: bad flag \"pins\"\n");
    row(&["units"], "user 5 call=c1 | x\n", 1, "", "windowfit: line 1: call needs role assistant\n");
    row(&["units"], "assistant 5 call=a call=b | x\n", 1, "", "windowfit: line 1: duplicate flag \"call=b\"\n");
    row(&["units"], "assistant 5 call=zzzzzzzzzzzz | x\n", 0, "1 assistant 5\n", "");
    row(&["units"], "assistant 5 call=zzzzzzzzzzzzz | x\n", 1, "", "windowfit: line 1: bad flag \"call=zzzzzzzzzzzzz\"\n");
    row(&["units"], "user 5 | ok\n\nuser x | bad\nbot 5 | worse\n", 1, "", "windowfit: line 3: bad tokens \"x\"\n");
}
