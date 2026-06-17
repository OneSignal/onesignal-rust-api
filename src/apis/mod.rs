use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> Error<T> {
    /// The error messages carried by the response body, normalized to a flat
    /// `Vec<String>` regardless of which envelope shape the API returned
    /// (`{"errors": "..."}`, `{"errors": ["..."]}`,
    /// `{"errors": [{"code": ..., "title": ...}]}`, or an object map such as
    /// `{"errors": {"invalid_aliases": {...}}}`, surfaced as `"<key>: <value>"`
    /// entries). Returns an empty `Vec` for non-response errors (network, serde,
    /// IO) or when the body is not a recognizable error envelope. The raw
    /// response remains available on the `ResponseError` variant.
    pub fn error_messages(&self) -> Vec<String> {
        let content = match self {
            Error::ResponseError(response) => &response.content,
            _ => return Vec::new(),
        };

        let parsed: serde_json::Value = match serde_json::from_str(content) {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };

        match parsed.get("errors") {
            Some(serde_json::Value::String(message)) => vec![message.clone()],
            Some(serde_json::Value::Array(items)) => items
                .iter()
                .filter_map(|item| match item {
                    serde_json::Value::String(message) => Some(message.clone()),
                    serde_json::Value::Object(object) => object
                        .get("title")
                        .filter(|value| !value.is_null() && value.as_str() != Some(""))
                        .or_else(|| object.get("code"))
                        .and_then(|value| match value {
                            serde_json::Value::String(message) => Some(message.clone()),
                            serde_json::Value::Null => None,
                            other => Some(other.to_string()),
                        }),
                    _ => None,
                })
                .collect(),
            Some(serde_json::Value::Object(map)) => {
                // Object-shaped envelopes (e.g. {"invalid_aliases": {...}}) carry
                // data under arbitrary keys; surface each so it isn't silently
                // dropped. Key order is unspecified, so sort for deterministic output.
                let mut messages: Vec<String> = map
                    .iter()
                    .map(|(key, value)| match value {
                        serde_json::Value::String(message) => format!("{}: {}", key, message),
                        other => format!("{}: {}", key, other),
                    })
                    .collect();
                messages.sort();
                messages
            }
            _ => Vec::new(),
        }
    }
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod default_api;

pub mod configuration;
