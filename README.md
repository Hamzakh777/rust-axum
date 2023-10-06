## Local dev
1. Start the app `cargo watch -q -c -w src/ -x run `
2. Open another terminal and run `cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"` (--nocapture allows to print)



## Notes
- In Rust, the idiomatic way to pass references to params is to pass a reference to the slice rather than to the object: https://rust-lang.github.io/rust-clippy/master/index.html#/ptr_arg