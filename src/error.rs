use serde::Deserialize;

// TODO https://lib.rs/crates/partial-enum
// Return subset of errors, that match exactly what the API can return

#[derive(Debug)]
pub enum Error {
    ApiUnavailable(String),
    MissingKey(String),
    BadKey(String),
    TooManyRequests(String),
    TooManyDecks(String),
    TooManyCardsInDeck(String),
    TooManyCardsTotal(String),
    BadDeck(String),
    BadVid(String),
    BadSid(String),
    BadRid(String),
    BadImage(String),
    BadAudio(String),
    BadRequest(String),
    BadSentence(String),
    BadTranslation(String),
    DeserializeError(std::io::Error),
    Transport(ureq::Transport),
    Unhandled(u16, RawError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    ApiUnavailable,
    MissingKey,
    BadKey,
    BadRequest,
    BadDeck,
    BadVid,
    BadSid,
    BadRid,
    BadImage,
    BadAudio,
    BadSentence,
    BadTranslation,
    TooManyRequests,
    TooManyDecks,
    TooManyCardsInDeck,
    TooManyCardsTotal,
    Transport,
    DeserializeError,
    Unhandled,
}

#[derive(Deserialize, Debug)]
pub struct RawError {
    error_message: String,
    error: String,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match *self {
            Error::MissingKey(_) => ErrorKind::MissingKey,
            Error::BadKey(_) => ErrorKind::BadKey,
            Error::BadRequest(_) => ErrorKind::BadRequest,
            Error::Transport(_) => ErrorKind::Transport,
            Error::Unhandled(_, _) => ErrorKind::Unhandled,
            Error::TooManyRequests(_) => ErrorKind::TooManyRequests,
            Error::ApiUnavailable(_) => ErrorKind::ApiUnavailable,
            Error::TooManyDecks(_) => ErrorKind::TooManyDecks,
            Error::TooManyCardsInDeck(_) => ErrorKind::TooManyCardsInDeck,
            Error::TooManyCardsTotal(_) => ErrorKind::TooManyCardsTotal,
            Error::BadDeck(_) => ErrorKind::BadDeck,
            Error::BadVid(_) => ErrorKind::BadVid,
            Error::BadSid(_) => ErrorKind::BadSid,
            Error::BadRid(_) => ErrorKind::BadRid,
            Error::BadImage(_) => ErrorKind::BadImage,
            Error::BadAudio(_) => ErrorKind::BadAudio,
            Error::BadSentence(_) => ErrorKind::BadSentence,
            Error::BadTranslation(_) => ErrorKind::BadTranslation,
            Error::DeserializeError(_) => ErrorKind::DeserializeError,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Transport(ref source) => Some(source),
            Error::DeserializeError(ref source) => Some(source),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::MissingKey(ref s) => write!(f, "No API key was specified. {s}"),
            Error::BadKey(ref s) => write!(f, "A bad API key was specified. {s}"),
            Error::Transport(ref e) => e.fmt(f),
            Error::BadRequest(ref s) => write!(f, "The request body did not match the schema. {s}"),
            Error::Unhandled(code, ref s) => write!(f, "Unhandled error. Code: {code}. Raw: {s:?}"),
            Error::TooManyRequests(ref s) => write!(f, "Too many requests. {s}"),
            Error::ApiUnavailable(ref s) => write!(f, "API Unavailable. {s}"),
            Error::TooManyDecks(ref s) => write!(f, "The user has too many decks. {s}"),
            Error::TooManyCardsInDeck(ref s) => {
                write!(f, "The user has too many cards in the given deck. {s}")
            }
            Error::TooManyCardsTotal(ref s) => {
                write!(f, "The user has reached the total card limit. {s}")
            }
            Error::BadDeck(ref s) => write!(f, "A deck with the given id doesn't exist. {s}"),
            Error::BadVid(ref s) => write!(f, "There is no vocabulary with the given id. {s}"),
            Error::BadSid(ref s) => write!(f, "There is no spelling with the given id. {s}"),
            Error::BadRid(ref s) => write!(f, "There is no reading with the given id. {s}"),
            Error::BadImage(ref s) => write!(f, "Bad image. {s}"),
            Error::BadAudio(ref s) => write!(f, "Bad audio. {s}"),
            Error::BadSentence(ref s) => write!(
                f,
                "The sentence is too long, or the given vocabulary was not found in it. {s}"
            ),
            Error::BadTranslation(ref s) => write!(f, "The translation is too long. {s}"),
            Error::DeserializeError(ref s) => s.fmt(f),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(r: ureq::Error) -> Self {
        match r {
            ureq::Error::Status(code, response) => {
                let raw = response.into_json::<RawError>().unwrap();

                match code {
                    403 => {
                        if raw.error != "bad_key" {
                            Error::Unhandled(code, raw)
                        } else {
                            match raw.error_message.as_str() {
                                "missing API key" => Error::MissingKey(raw.error_message),
                                "invalid API key" => Error::BadKey(raw.error_message),
                                _ => Error::Unhandled(code, raw),
                            }
                        }
                    }
                    429 => {
                        if raw.error != "too_many_requests" {
                            Error::Unhandled(code, raw)
                        } else {
                            //TODO uhh wait until 429 is implemented to match the string properly
                            Error::TooManyRequests(raw.error_message)
                        }
                    }
                    400 => match raw.error_message.as_str() {
                        "bad_request" => Error::BadRequest(raw.error_message),
                        _ => Error::Unhandled(code, raw),
                    },
                    _ => Error::Unhandled(code, raw),
                }
            }
            ureq::Error::Transport(a) => Error::Transport(a),
        }
    }
}
