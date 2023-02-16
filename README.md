# jpdb-rs

This is a Rust wrapper for [jpdb.io](https://www.jpdb.io/)'s API.

It uses [ureq](https://lib.rs/crates/ureq#readme-blocking-io-for-simplicity) and offers syncronous api calls by design, mainly because no client would make enough calls at the same time for it to reasonably matter. This also simplifies use, reduces compile time/executable size, and massively simplifies the dependency tree. If you need to make calls in an async function, you can turn to your executor's function of choice (eg. tokio's `spawn_blocking`) instead.

## How to use

```rust
use jpdb::client::Client;

fn main() {
    // First, we create a client
    let client = Client::new("my_jpdb_token"); //token is stored in the settings page

    // Next, we can call upon the api easily
    let response = client.ping();

    //Every function returns a result, with any error type jpdb can return
    if let Err(jpdb::error::Error::BadKey(__)) = response {
        println!("your error handling here, because the token was bad");
    }

    // That's about it! Read the docs for all available endpoints, and the error they'd return.

    // Some endpoints are a little more complicated, so they take in a struct as argument
    client.set_card_sentence(&SetCardSentenceOptions{
        vid: Vid(12),
        sid: Sid(10),
        clear_audio: Some(true),
        ..Default::default()
    });

}
```

For more info, refer to [this project](https://github.com/sdbversini/jpdb-tools) for practical use.

## Semantic Versioning

Like all rust crates are advised to, `jpdb` uses semantic versioning. The rationale is the following, for X.Y.Z:

- X represents the jpdb api version, eg. "/api/vX/ping". Expect this to stay at 1 indefinitely.
- Y represents breaking or semi-breaking changes, for instance, new or changed error types, new struct fields, renamed functions, and so on. A Changelog will be updated to reflect changes and help transition.
- Z will denote any minor update, to docs or internals.
