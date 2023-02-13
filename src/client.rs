use std::{fmt::Display, time::Duration};

use ureq::{Agent, Error, Response};

use crate::request::Request;

#[derive(Debug, Clone)]
pub struct Client {
    // TODO this might be redundant?
    _token: String,
    bearer: String,
    agent: Agent,
    #[cfg(test)]
    prefer: Option<String>,
    pub(crate) base_url: &'static str,
}

impl Client {
    fn create_agent() -> Agent {
        ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(20))
            .timeout_write(Duration::from_secs(5))
            .build()
    }

    pub(crate) fn create_url(base_url: &'static str, path: &str) -> String {
        //Yes, absurd levels of optimisation, but this makes up for the mocking server
        let mut r = String::with_capacity(130);
        r.push_str(base_url);
        r.push_str(path);
        r
    }

    pub fn new(token: &str) -> Self {
        Self {
            _token: token.to_owned(),
            bearer: format!("Bearer {token}"),
            agent: Self::create_agent(),
            base_url: "https://jpdb.io/api/v1/",
            #[cfg(test)]
            prefer: None,
        }
    }

    #[cfg(test)]
    pub fn new_mock(token: &str, prefer: Option<String>) -> Self {
        Self {
            _token: token.to_owned(),
            bearer: format!("Bearer {token}"),
            agent: Self::create_agent(),
            prefer,
            base_url: "https://stoplight.io/mocks/jpdb/jpdb/125397907/api/v1/",
        }
    }

    #[cfg(not(test))]
    pub(crate) fn send_request(&self, prepared: Request) -> Result<Response, Error> {
        self.agent
            .post(&prepared.url)
            .set("Authorization", &self.bearer)
            .send_json(prepared.body)
    }

    #[cfg(test)]
    pub(crate) fn send_request(&self, prepared: Request) -> Result<Response, Error> {
        let mut request = self
            .agent
            .post(&prepared.url)
            .set("Authorization", &self.bearer);
        if let Some(ref p) = self.prefer {
            request = request.set("Prefer", p);
        }
        // Fixes ping not working in the mock server
        if prepared.body == serde_json::Value::Null {
            request.call()
        } else {
            request.send_json(prepared.body)
        }
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jpdb client, token: {}", self.bearer)
    }
}
