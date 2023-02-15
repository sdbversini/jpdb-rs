use std::env;

use jpdb::{
    client::Client,
    request::{DeckVocabulary, UserDeckId},
};

fn get_good_client() -> Client {
    Client::new(&env::var("JPDB_TOKEN").unwrap_or_else(|_| String::from("")))
}

#[test]
fn jpdb_bad_key() {
    let c = Client::new("badtoken");
    let result = c.ping().unwrap_err();
    assert_eq!(result.kind(), jpdb::error::ErrorKind::BadKey);
}

#[test]
fn jpdb_no_key() {
    let c = Client::new("");
    let result = c.ping().unwrap_err();
    assert_eq!(result.kind(), jpdb::error::ErrorKind::MissingKey);
}

#[test]
fn jpdb_good_key() {
    let c = get_good_client();
    let result = c.ping();
    assert!(result.is_ok());
}

#[ignore]
#[test]
fn jpdb_clear_blacklist() {
    let c = get_good_client();
    let result = c.clear_deck(jpdb::request::SpecialDeckId::Blacklist);
    assert!(result.is_ok());
}

#[ignore]
#[test]
fn jpdb_clear_neverforget() {
    let c = get_good_client();
    let result = c.clear_deck(jpdb::request::SpecialDeckId::NeverForget);
    assert!(result.is_ok());
}

#[ignore]
#[test]
fn jpdb_clear_id() {
    let c = get_good_client();
    let result = c.clear_deck(jpdb::request::UserDeckId(3));
    assert!(result.is_ok());
}

#[ignore]
#[test]
fn jpdb_create_delete_deck() {
    // TODO create deck, get id, delete that id, list decks, verify it's not there
}

#[ignore]
#[test]
// TODO still not implemented server side
// keep ignored, no need to spam the server with every test batch
fn jpdb_429() {
    // assert!(false)
    let c = Client::new("badtoken");
    for _ in 0..50 {
        let result = c.ping().unwrap_err();
        dbg!(&result);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[ignore]
#[test]
// TODO test occurence none when false
fn jpdb_list_vocabulary_raw_some_false() {
    let c = get_good_client();
    let resp = c.list_vocabulary_raw(UserDeckId(12), Some(false));
    assert!(resp.is_ok());
    assert_eq!(
        resp.unwrap(),
        DeckVocabulary {
            vocabulary: vec![vec![0]],
            occurences: None,
        }
    )
}
