use crate::{
    client::Client,
    request::{
        AddVocabularyOptions, DeckQueryField, DeckVocabulary, SetCardSentenceOptions, Sid,
        SpecialDeckId, TokenQueryField, UserDeckId, Vid, VocabQueryField, Vocabulary,
    },
};

#[test]
fn post_ping_ok() {
    let client = Client::new_mock("aaa", Some(String::from("code=200")));
    let resp = client.ping();
    assert!(resp.is_ok());
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

#[test]
fn mock_delete_deck() {
    let client = Client::new_mock("aaa", None);
    let resp = client.delete_deck(UserDeckId(1));
    assert!(resp.is_ok());
}

#[test]
fn mock_rename_deck() {
    let client = Client::new_mock("aaa", None);
    let resp = client.rename_deck(UserDeckId(1), "asa");
    assert!(resp.is_ok());
}

//Ignored until the update that fixes the endpoint url
#[ignore]
#[test]
fn mock_set_card_sentence() {
    let client = Client::new_mock("aaa", None);
    let options = SetCardSentenceOptions {
        sentence: Some(""),
        translation: Some(""),
        clear_audio: Some(false),
        clear_image: Some(false),
        ..Default::default()
    };
    let resp = client.set_card_sentence(&options);
    dbg!(&resp);
    assert!(resp.is_ok());
}

//Ignored until the update that fixes the endpoint url
#[ignore]
#[test]
fn mock_set_card_sentence_none() {
    let client = Client::new_mock("aaa", None);
    let options = SetCardSentenceOptions {
        ..Default::default()
    };
    let resp = client.set_card_sentence(&options);
    assert!(resp.is_ok());
}

#[test]
fn mock_remove_vocabulary() {
    let client = Client::new_mock("aaa", None);
    let resp = client.remove_vocabulary(UserDeckId(1), &[Vocabulary(12, 12), Vocabulary(13, 13)]);
    assert!(resp.is_ok());
}

#[test]
fn mock_remove_neverforget() {
    let client = Client::new_mock("aaa", None);
    let resp = client.remove_vocabulary(
        SpecialDeckId::NeverForget,
        &[Vocabulary(12, 12), Vocabulary(13, 13)],
    );
    assert!(resp.is_ok());
}

#[test]
fn mock_remove_blacklist() {
    let client = Client::new_mock("aaa", None);
    let resp = client.remove_vocabulary(
        SpecialDeckId::Blacklist,
        &[Vocabulary(12, 12), Vocabulary(13, 13)],
    );
    assert!(resp.is_ok());
}

#[test]
fn mock_add_vocab_blacklist() {
    let client = Client::new_mock("aaa", None);
    let resp = client.add_vocabulary(
        SpecialDeckId::Blacklist,
        &AddVocabularyOptions {
            vocabulary: &[Vocabulary(12, 12), Vocabulary(13, 13)],
            ..Default::default()
        },
    );
    assert!(resp.is_ok());
}

#[test]
fn mock_add_vocab_user() {
    let client = Client::new_mock("aaa", None);
    let resp = client.add_vocabulary(
        UserDeckId(12),
        &AddVocabularyOptions {
            vocabulary: &[Vocabulary(13, 13)],
            occurences: Some(&[1, 1]),
            overwrite_occurences: Some(true),
            ignore_unknown: Some(false),
        },
    );
    assert!(resp.is_ok());
}

#[test]
fn mock_create_deck() {
    let client = Client::new_mock("aaa", None);
    let resp = client.create_empty_deck("baba", None);
    dbg!(&resp);
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), UserDeckId(0))
}

#[test]
fn mock_create_deck_some() {
    let client = Client::new_mock("aaa", None);
    let resp = client.create_empty_deck("baba", Some(1));
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), UserDeckId(0))
}

#[test]
fn mock_list_special_decks() {
    let client = Client::new_mock("aaa", None);
    let resp = client.list_special_decks(&[
        DeckQueryField::KnownCoverage,
        DeckQueryField::Id,
        DeckQueryField::Name,
        DeckQueryField::VocabularyCount,
        DeckQueryField::InProgressCoverage,
        DeckQueryField::WordCount,
        DeckQueryField::IsBuiltIn,
    ]);
    assert!(resp.is_ok());
}

#[test]
fn mock_list_user_decks() {
    let client = Client::new_mock("aaa", None);
    let resp = client.list_user_decks(&[
        DeckQueryField::Id,
        DeckQueryField::Id,
        DeckQueryField::Id,
        DeckQueryField::Id,
        DeckQueryField::Id,
        DeckQueryField::Id,
        DeckQueryField::InProgressCoverage,
        DeckQueryField::IsBuiltIn,
        DeckQueryField::Id,
        DeckQueryField::KnownCoverage,
        DeckQueryField::Name,
        DeckQueryField::VocabularyCount,
        DeckQueryField::WordCount,
    ]);
    assert!(resp.is_ok());
}

#[test]
fn mock_list_all_decks() {
    let client = Client::new_mock("aaa", None);
    let resp = client.list_user_decks(&[
        DeckQueryField::Id,
        DeckQueryField::InProgressCoverage,
        DeckQueryField::IsBuiltIn,
        DeckQueryField::KnownCoverage,
        DeckQueryField::Name,
        DeckQueryField::VocabularyCount,
        DeckQueryField::WordCount,
    ]);
    assert!(resp.is_ok());
}

#[test]
fn mock_lookup_vocab() {
    let client = Client::new_mock("aaa", None);
    let resp = client.lookup_vocabulary(
        &[(Vid(0), Sid(0))],
        &[
            VocabQueryField::CardLevel,
            VocabQueryField::CardState,
            VocabQueryField::DueAt,
            VocabQueryField::FrequencyRank,
            VocabQueryField::Meanings,
            VocabQueryField::Reading,
            VocabQueryField::Rid,
            VocabQueryField::Sid,
            VocabQueryField::Spelling,
            VocabQueryField::Vid,
        ],
    );
    assert!(resp.is_ok());
}

#[test]
fn mock_parse_text() {
    let client = Client::new_mock("aaa", None);
    let resp = client.parse_text(
        "scouchou",
        &[
            TokenQueryField::Furigana,
            TokenQueryField::LengthUtf32,
            TokenQueryField::LengthUtf8,
            TokenQueryField::PositionUtf32,
            TokenQueryField::PositionUtf8,
            TokenQueryField::VocabIndex,
        ],
        Some(&[
            VocabQueryField::CardLevel,
            VocabQueryField::CardState,
            VocabQueryField::DueAt,
            VocabQueryField::FrequencyRank,
            VocabQueryField::Meanings,
            VocabQueryField::Reading,
            VocabQueryField::Rid,
            VocabQueryField::Sid,
            VocabQueryField::Spelling,
            VocabQueryField::Vid,
        ]),
    );
    dbg!(&resp);
    assert!(resp.is_ok());
}
