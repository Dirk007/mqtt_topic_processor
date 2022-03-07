use std::{borrow::Cow, fmt::Debug};

use anyhow::{anyhow, Result};
use serde::Deserialize;

/// A message from and to MQTT containing the topic and the payload which is assumed to be JSON.
/// Topic and json are held as [Cow] to keep cloning around to a minimum / where really needed.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JsonMessage<'a> {
    /// The topic
    pub topic: Cow<'a, str>,
    /// The message-payload
    pub json: Cow<'a, str>,
}

impl<'a, T, P> From<(&'a T, &'a P)> for JsonMessage<'a>
where
    T: AsRef<str> + ?Sized,
    P: AsRef<str> + ?Sized,
{
    fn from((topic, payload): (&'a T, &'a P)) -> Self {
        Self {
            topic: Cow::Borrowed(topic.as_ref()),
            json: Cow::Borrowed(payload.as_ref()),
        }
    }
}

/// DRY removal trait that also forces one to use Type::decode() for deserialization using
/// explicit type annotation but easier(tm).
/// As there is still a weakness in Rust which makes implicit type inferrence on a Result-
/// target assuming Unit "()"-output there is a good chance that the result is not the expected one elseway.
pub trait JsonDeserializable<I, O> {
    fn decode(input: I) -> Result<O>;
}

impl<I, O> JsonDeserializable<I, O> for O
where
    I: AsRef<str>,
    O: for<'de> Deserialize<'de>,
{
    fn decode(input: I) -> Result<O> {
        serde_json::from_reader(input.as_ref().as_bytes())
            .map_err(|e| anyhow!("Json decode error: {:?} in {}", e, input.as_ref()))
    }
}
