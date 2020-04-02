Test sentry + tide + async-std


Issue: 
Random segmentation fault when a panic is called in a tide request handler.


**How to reproduce:**

Start the test
```bash
$ cargo run
```

Call the route that will panic
```bash
$ curl "http://localhost:8080/"
```

**How to debug:**

Build binary and start LLDB
```bash
$ cargo build
$ rust-lldb target/debug/test-sentry
```

Call the route that will panic
```bash
$ curl "http://localhost:8080/"
```

In lldb:
- `r`: run the binary
- `up`: wind up the backtrace
- `q`: quit


Re-run the program and call the route until the program have a segfault.

The backtrace is in the [export-lldb file](./export-lldb.txt)
