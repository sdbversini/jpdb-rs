use std::num::NonZeroU16;

use serde_json::json;

use crate::{client::Client, error::Error};

/// A request ready to be sent, users of the crate won't have to touch anything in here normally.
#[derive(Debug)]
pub struct Request {
    /// The full URL that the request will be sent to, this includes the base URL and the API endpoint
    pub url: String,
    /// The body of the request
    pub body: serde_json::Value,
}

pub enum AnyDeckWidget {
    DeckId(NonZeroU16),
    Blacklist,
    NeverForget,
}

// #[derive(serde::Serialize)]
pub trait AnyDeckId {
    fn as_any(&self) -> AnyDeckWidget;
}

impl serde::Serialize for AnyDeckWidget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            AnyDeckWidget::DeckId(x) => serializer.serialize_u16(x.get()),
            AnyDeckWidget::NeverForget => serializer.serialize_str("never-forget"),
            AnyDeckWidget::Blacklist => serializer.serialize_str("blacklist"),
        }
    }
}

#[derive(serde::Serialize)]
pub struct UserDeckId(pub NonZeroU16);
#[derive(serde::Serialize)]
pub enum SpecialDeckId {
    Blacklist,
    NeverForget,
}

impl AnyDeckId for UserDeckId {
    fn as_any(&self) -> AnyDeckWidget {
        AnyDeckWidget::DeckId(self.0)
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
}
