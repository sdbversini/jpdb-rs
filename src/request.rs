use serde::Serialize;
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
    UserDeckId(u16),
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
        S: serde::Serializer,
    {
        match *self {
            AnyDeckWidget::UserDeckId(x) => serializer.serialize_u16(x),
            AnyDeckWidget::NeverForget => serializer.serialize_str("never-forget"),
            AnyDeckWidget::Blacklist => serializer.serialize_str("blacklist"),
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vid(pub u16);
#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rid(pub u16);
#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Sid(pub u16);

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

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct UserDeckId(pub u16);
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

impl Client {
    pub fn ping(&self) -> Result<(), Error> {
        let request = Request {
            url: Client::create_url(self.base_url, "ping"),
            body: serde_json::Value::Null,
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
