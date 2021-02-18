## Building 
Just invoke
```bash 
cargo build 
```

## Execution
```bash
./target/debug/racecondition
```

## About 
The author of this slightly modified program is Adithya B.
This is learning material used at the Course USP and CD Lab at the Jyothy Institue of Technology.

The initial C program was designed to produce a race condition for long strings that are printed character by character,
eventually fighting for the same pid_t.

Try it out by repeatedly executing the program, although for me it often produces the issue right away on the first try.

**In this example we show that a transpilation with c2rust does not solve this issue and we can produce the exact same race condition. So even though our code is now transpiled into Rust, it suffers from the same obvious weakness as the initial C program.**

