use std::borrow::Cow;

use anyhow::Result;
use async_trait::async_trait;

use crate::{JsonMessage, JsonTopicProcessor, SimpleState, StateValue};

/// Simply forwards a message on the registered input-topic to a given output-topic.
pub struct Forwarder {
    state: SimpleState,
    /// The topic we forward incoming messages to
    topic: Cow<'static, str>,
}

impl Forwarder {
    /// Create a new [JsonTopicProcessor] which forwards all messages it receives
    /// to the given `topic` as the processing-result.
    pub fn new(topic: Cow<'static, str>) -> Self {
        Self {
            state: SimpleState::default(),
            topic,
        }
    }
}

#[async_trait]
impl JsonTopicProcessor for Forwarder {
    async fn process(&self, input: Cow<'static, str>) -> Result<Option<Vec<JsonMessage>>> {
        if !self.is_running() {
            return Ok(None);
        }

        let result = JsonMessage {
            topic: self.topic.clone(),
            json: input,
        };

        Ok(Some(vec![result]))
    }

    fn state(&self) -> &StateValue {
        self.state.as_ref()
    }

    fn set_state(&mut self, new_state: StateValue) {
        *self.state.as_mut() = new_state;
    }
}
