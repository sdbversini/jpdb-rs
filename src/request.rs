use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;

use crate::{client::Client, error::Error};

/// A request ready to be sent, users of the crate won't have to touch anything in here normally.
#[derive(Debug)]
pub(crate) struct Request {
    /// The full URL that the request will be sent to, this includes the base URL and the API endpoint
    pub url: String,
    /// The body of the request
    pub body: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AnyDeckWidget {
    UserDeckId(u8),
    Blacklist,
    NeverForget,
}

// #[derive(serde::Serialize)]
pub trait AnyDeckId {
    fn as_any(&self) -> AnyDeckWidget;
}

impl Serialize for AnyDeckWidget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AnyDeckWidget::UserDeckId(x) => serializer.serialize_u8(x),
            AnyDeckWidget::NeverForget => serializer.serialize_str("never-forget"),
            AnyDeckWidget::Blacklist => serializer.serialize_str("blacklist"),
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vid(pub u32);
#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rid(pub u32);
#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Sid(pub u32);

type Vocabulary = (Vid, Sid);

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct SetCardSentenceOptions<'a> {
    pub vid: Vid,
    pub sid: Sid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_image: Option<bool>,
}

impl Default for SetCardSentenceOptions<'_> {
    fn default() -> Self {
        Self {
            vid: Vid(0),
            sid: Sid(0),
            sentence: None,
            translation: None,
            clear_audio: None,
            clear_image: None,
        }
    }
}

#[derive(Serialize, Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct AddVocabularyOptions<'a> {
    pub vocabulary: &'a [Vocabulary],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurences: Option<&'a [u16]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_occurences: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unknown: Option<bool>,
}

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct UserDeckId(pub u8);
#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum SpecialDeckId {
    Blacklist,
    NeverForget,
}

impl AnyDeckId for UserDeckId {
    fn as_any(&self) -> AnyDeckWidget {
        AnyDeckWidget::UserDeckId(self.0)
    }
}
impl AnyDeckId for SpecialDeckId {
    fn as_any(&self) -> AnyDeckWidget {
        match self {
            SpecialDeckId::Blacklist => AnyDeckWidget::Blacklist,
            SpecialDeckId::NeverForget => AnyDeckWidget::NeverForget,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DeckVocabulary {
    pub vocabulary: Vec<Vec<u32>>,
    pub occurences: Option<Vec<u32>>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct CreateEmptyDeckResponse {
    id: u8,
}

impl From<CreateEmptyDeckResponse> for UserDeckId {
    fn from(x: CreateEmptyDeckResponse) -> Self {
        Self(x.id)
    }
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeckQueryField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "vocabulary_count")]
    VocabularyCount,
    #[serde(rename = "word_count")]
    WordCount,
    #[serde(rename = "vocabulary_known_coverage")]
    KnownCoverage,
    #[serde(rename = "vocabulary_in_progress_coverage")]
    InProgressCoverage,
    #[serde(rename = "is_built_in")]
    IsBuiltIn,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VocabQueryField {
    #[serde(rename = "vid")]
    Vid,
    #[serde(rename = "sid")]
    Sid,
    #[serde(rename = "rid")]
    Rid,
    #[serde(rename = "spelling")]
    Spelling,
    #[serde(rename = "reading")]
    Reading,
    #[serde(rename = "frequency_rank")]
    FrequencyRank,
    #[serde(rename = "meanings")]
    Meanings,
    #[serde(rename = "card_level")]
    CardLevel,
    #[serde(rename = "card_state")]
    CardState,
    #[serde(rename = "due_at")]
    DueAt,
}

//TODO remove utf32?
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenQueryField {
    #[serde(rename = "vocabulary_index")]
    VocabIndex,
    #[serde(rename = "position_utf8")]
    PositionUtf8,
    #[serde(rename = "position_utf32")]
    PositionUtf32,
    #[serde(rename = "length_utf8")]
    LengthUtf8,
    #[serde(rename = "length_utf32")]
    LengthUtf32,
    #[serde(rename = "furigana")]
    Furigana,
}

pub enum DeckTypeToList {
    UserDecks,
    SpecialDecks,
}

impl DeckTypeToList {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DeckTypeToList::UserDecks => "list-user-decks",
            DeckTypeToList::SpecialDecks => "list-special-decks",
        }
    }
}

impl Client {
    pub fn ping(&self) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "ping"),
            body: serde_json::Value::Null,
        };
        self.send_request(request)?;
        Ok(())
    }

    pub fn parse_text(
        &self,
        text: &str,
        token_fields: &[TokenQueryField],
        vocab_fields: Option<&[VocabQueryField]>,
    ) -> Result<(), Error> {
        let mut token_fields = token_fields.to_vec();
        // fields.sort(); // TODO?
        token_fields.dedup();

        let body = if let Some(fields) = vocab_fields {
            let mut vocab_fields = fields.to_vec();
            // fields.sort(); // TODO?
            vocab_fields.dedup();
            json!({
                "text": text,
                "token_fields": token_fields,
                "vocabulary_fields": vocab_fields,
            })
        } else {
            json!({
                "text": text,
                "token_fields": token_fields,
            })
        };

        let request = Request {
            url: Client::create_url(self.base_url, "parse"),
            body,
        };
        let response = self
            .send_request(request)?
            .into_json::<serde_json::Value>()
            .map_err(Error::DeserializeError)?;
        dbg!(response);
        //TODO deserialize
        unimplemented!()
    }

    pub fn lookup_vocabulary(
        &self,
        list: &[(Vid, Sid)],
        fields: &[VocabQueryField],
    ) -> Result<(), Error> {
        let mut fields = fields.to_vec();
        // fields.sort(); // TODO?
        fields.dedup();
        let request = Request {
            url: Client::create_url(self.base_url, "lookup-vocabulary"),
            body: json! ({
                "list": list,
                "fields": fields,
            }),
        };
        let _response = self
            .send_request(request)?
            .into_json::<serde_json::Value>()
            .map_err(Error::DeserializeError)?;
        //TODO deserialize
        unimplemented!()
    }

    pub fn list_decks_raw(
        &self,
        fields: &[DeckQueryField],
        deck_type: DeckTypeToList,
    ) -> Result<(), Error> {
        let mut fields = fields.to_vec();
        // fields.sort(); // TODO?
        fields.dedup();
        let _request = Request {
            url: Client::create_url(self.base_url, deck_type.as_str()),
            body: json! ({
                "fields": fields,
            }),
        };
        // TODO destructure the reply
        unimplemented!();
        // Ok(())
    }

    pub fn list_special_decks(&self, fields: &[DeckQueryField]) -> Result<(), Error> {
        let _decks = self.list_decks_raw(fields, DeckTypeToList::SpecialDecks)?;
        unimplemented!();
        // Ok(())
    }

    pub fn list_user_decks(&self, fields: &[DeckQueryField]) -> Result<(), Error> {
        let _decks = self.list_decks_raw(fields, DeckTypeToList::UserDecks)?;
        unimplemented!();
        // Ok(())
    }

    pub fn list_all_decks(&self, _fields: &[DeckQueryField]) -> Result<(), Error> {
        unimplemented!();
        // Ok(())
    }

    pub fn create_empty_deck(&self, name: &str, position: Option<u8>) -> Result<UserDeckId, Error> {
        let body = if let Some(p) = position {
            json!({"name": name, "position": p})
        } else {
            json!({ "name": name })
        };
        let request = Request {
            url: Client::create_url(self.base_url, "deck/create-empty"),
            body,
        };
        let response = self
            .send_request(request)?
            .into_json::<CreateEmptyDeckResponse>()
            .map_err(Error::DeserializeError)?;
        Ok(response.into())
    }

    pub fn list_vocabulary_raw(
        &self,
        deck_id: impl AnyDeckId,
        fetch_occurence: Option<bool>,
    ) -> Result<DeckVocabulary, Error> {
        // TODO remove fetch_occurence from body if None
        let request = Request {
            url: Client::create_url(self.base_url, "deck/list-vocabulary"),
            body: json!({
                "id": deck_id.as_any(),
                "fetch_occurence": fetch_occurence
            }),
        };
        let response = self
            .send_request(request)?
            .into_json::<DeckVocabulary>()
            .map_err(Error::DeserializeError)?;
        Ok(response)
    }

    pub fn list_vocabulary(
        &self,
        deck_id: impl AnyDeckId,
        fetch_occurence: Option<bool>,
    ) -> Result<(), Error> {
        let _raw = self.list_vocabulary_raw(deck_id, fetch_occurence)?;
        //TODO turn into raw
        // dolater when we know the proper structure
        unimplemented!("unimplemented for now, waiting for API release to handle correctly");
    }

    pub fn add_vocabulary(
        &self,
        deck_id: impl AnyDeckId,
        options: &AddVocabularyOptions,
    ) -> Result<(), Error> {
        let mut body = json!(options).as_object_mut().unwrap().clone();
        body.insert("id".to_string(), json!(deck_id.as_any()));
        let request = Request {
            url: Client::create_url(self.base_url, "deck/add-vocabulary"),
            body: json!(body),
        };
        self.send_request(request)?;
        Ok(())
    }

    pub fn remove_vocabulary(
        &self,
        deck_id: impl AnyDeckId,
        vocabulary: &[Vocabulary],
    ) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "deck/remove-vocabulary"),
            body: json!({
                "id": deck_id.as_any(),
                "vocabulary": vocabulary,
            }),
        };
        self.send_request(request)?;
        Ok(())
    }

    pub fn clear_deck(&self, deck_id: impl AnyDeckId) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "deck/clear"),
            body: json!({
                "id": deck_id.as_any(),
            }),
        };
        self.send_request(request)?;
        Ok(())
    }

    pub fn delete_deck(&self, deck_id: UserDeckId) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "deck/delete"),
            body: json!({
                "id": deck_id.as_any(),
            }),
        };
        self.send_request(request)?;
        Ok(())
    }

    pub fn rename_deck(&self, deck_id: UserDeckId, new_name: &str) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "deck/delete"),
            body: json!({
                "id": deck_id.as_any(),
                "name": new_name,
            }),
        };
        self.send_request(request)?;
        Ok(())
    }

    pub fn set_card_sentence(&self, options: &SetCardSentenceOptions) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "set-card-sentence"),
            body: json!(options),
        };
        self.send_request(request)?;
        Ok(())
    }
}
