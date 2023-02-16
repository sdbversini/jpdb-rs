use std::{env, time::Duration};

use jpdb::{
    client::Client,
    error::ErrorKind,
    request::{
        AddVocabularyOptions, DeckQueryField, DeckVocabulary, SetCardSentenceOptions, Sid,
        UserDeckId, Vid,
    },
};

fn get_good_client() -> Client {
    std::thread::sleep(Duration::from_secs(5));
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

#[test]
fn jpdb_clear_blacklist() {
    let c = get_good_client();
    let result = c.clear_deck(jpdb::request::SpecialDeckId::Blacklist);
    assert!(result.is_ok());
}

#[test]
fn jpdb_clear_neverforget() {
    let c = get_good_client();
    let result = c.clear_deck(jpdb::request::SpecialDeckId::NeverForget);
    assert!(result.is_ok());
}

#[test]
fn jpdb_clear_id() {
    let c = get_good_client();
    let result = c.clear_deck(jpdb::request::UserDeckId(3));
    assert!(result.is_ok());
}

#[test]
fn jpdb_create_delete_deck() {
    // TODO create deck, get id, delete that id, list decks, verify it's not there
}

#[ignore]
#[test]
fn jpdb_delete_deck() {
    let c = get_good_client();
    let result = c.delete_deck(jpdb::request::UserDeckId(5));
    assert!(result.is_ok());
}

// #[ignore]
// #[test]
// // TODO still not implemented server side
// // keep ignored, no need to spam the server with every test batch
// fn jpdb_429() {
//     // assert!(false)
//     let c = Client::new("badtoken");
//     for _ in 0..50 {
//         let result = c.ping().unwrap_err();
//         dbg!(&result);
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }

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

#[test]
fn jpdb_create_empty_deck_no_name() {
    let c = get_good_client();
    let resp = c.create_empty_deck("", None);
    assert!(resp.is_err());
    assert_eq!(resp.unwrap_err().kind(), jpdb::error::ErrorKind::BadRequest);
}

#[test]
fn jpdb_create_empty_deck_huge_position() {
    let c = get_good_client();
    let resp = c.create_empty_deck("taratata", Some(200));
    assert!(resp.is_ok());
}

#[test]
fn jpdb_rename_deck() {
    let c = get_good_client();
    let resp = c.rename_deck(UserDeckId(7), "renamed");
    assert!(resp.is_ok());
}

#[test]
fn jpdb_rename_deck_no_name() {
    let c = get_good_client();
    let resp = c.rename_deck(UserDeckId(7), "");
    assert!(resp.is_err());
}

#[test]
fn jpdb_add_vocabulary() {
    let c = get_good_client();
    let resp = c.add_vocabulary(
        UserDeckId(7),
        &AddVocabularyOptions {
            vocabulary: &[(Vid(1358280), Sid(1232985445))],
            occurences: Some(&[10]),
            overwrite_occurences: Some(true),
            ignore_unknown: Some(true),
        },
    );
    assert!(resp.is_ok());
}

#[test]
fn jpdb_add_vocabulary_too_many() {
    let c = get_good_client();
    let resp = c.add_vocabulary(
        UserDeckId(7),
        &AddVocabularyOptions {
            vocabulary: &[(Vid(1358280), Sid(1232985445))],
            occurences: Some(&[10, 15]),
            overwrite_occurences: Some(true),
            ignore_unknown: Some(true),
        },
    );
    assert!(resp.is_err());
}

#[test]
fn jpdb_add_vocabulary_ignore() {
    let c = get_good_client();
    let resp = c.add_vocabulary(
        UserDeckId(7),
        &AddVocabularyOptions {
            vocabulary: &[(Vid(1358280), Sid(1))],
            occurences: Some(&[15]),
            overwrite_occurences: Some(true),
            ignore_unknown: Some(false),
        },
    );
    assert!(resp.is_err());
    assert_eq!(resp.unwrap_err().kind(), ErrorKind::BadSid);

    let resp = c.add_vocabulary(
        UserDeckId(7),
        &AddVocabularyOptions {
            vocabulary: &[(Vid(1358280), Sid(1))],
            occurences: Some(&[15]),
            overwrite_occurences: Some(true),
            ignore_unknown: Some(true),
        },
    );
    assert!(resp.is_ok());
}

#[test]
fn jpdb_add_remove_vocabulary() {
    let word = (Vid(1555480), Sid(2996971705));
    let c = get_good_client();
    let resp = c.add_vocabulary(
        UserDeckId(7),
        &AddVocabularyOptions {
            vocabulary: &[word],
            occurences: Some(&[1578]),
            overwrite_occurences: Some(true),
            ignore_unknown: Some(false),
        },
    );
    assert!(resp.is_ok());
    let resp = c.remove_vocabulary(UserDeckId(7), &[word]);
    assert!(resp.is_ok());
}

#[ignore]
#[test]
fn jpdb_remove_vocabulary() {
    let word = (Vid(2028920), Sid(2204744690));
    let c = get_good_client();
    let resp = c.remove_vocabulary(UserDeckId(7), &[word]);
    assert!(resp.is_ok());
}

#[test]
fn jpdb_set_card_sentence() {
    let c = get_good_client();
    let resp = c.set_card_sentence(&SetCardSentenceOptions {
        vid: Vid(1310890),
        sid: Sid(1197989957),
        sentence: Some("babawo死神"),
        translation: None,
        clear_audio: None,
        clear_image: None,
    });
    dbg!(&resp);
    assert!(&resp.is_ok());
}

//TODO test list-special-decks
// - no fields
// - duplicated fields
