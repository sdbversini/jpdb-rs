- jpdb-rs
  This is a Rust wrapper for [jpdb.io](https://www.jpdb.io/)'s API.

It uses [ureq](https://lib.rs/crates/ureq#readme-blocking-io-for-simplicity) and offers syncronous api calls by design, mainly because no client would make enough calls at the same time for it to reasonably matter. This also simplifies use, reduces compile time/executable size, and massively simplifies the dependency tree. If you need to make calls in an async function, you can turn to your executor's function of choice (eg. tokio's ==spawn_blocking==) instead.
