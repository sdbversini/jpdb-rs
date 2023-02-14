use crate::{
    client::Client,
    request::{SpecialDeckId, UserDeckId},
};

#[test]
fn post_ping_ok() {
    let client = Client::new_mock("aaa", Some(String::from("code=200")));
    let resp = client.ping();
    assert!(resp.is_ok());
}

#[test]
fn too_many_requests() {
    // Mock server doesn't reply the right thing for 429 errors, so ignore for now
    // let client = Client::new_mock("aaa", Some(String::from("code=429")));
    // client.ping();
    // tododo
}

#[test]
fn mock_clear_deck_blacklist() {
    let client = Client::new_mock("aaa", None);
    let resp = client.clear_deck(SpecialDeckId::Blacklist);
    assert!(resp.is_ok());
}

#[test]
fn mock_clear_deck_never_forget() {
    let client = Client::new_mock("aaa", None);
    let resp = client.clear_deck(SpecialDeckId::NeverForget);
    assert!(resp.is_ok());
}
