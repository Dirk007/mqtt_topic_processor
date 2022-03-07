use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
};

use anyhow::Result;
use async_trait::async_trait;

use crate::{json_message::JsonMessage, state::StateValue};

/// Processes a topic, optionally returning messages as a reply
#[async_trait]
pub trait JsonTopicProcessor {
    /// Process the `input` for the topic this Processor was registered for and optionally return
    /// messages in reply.
    async fn process<'t>(&'t self, input: Cow<'t, str>) -> Result<Option<Vec<JsonMessage<'t>>>>;

    fn state(&self) -> &StateValue;
    fn set_state(&mut self, new_state: StateValue);

    fn is_running(&self) -> bool {
        self.state() == &StateValue::Running
    }

    fn is_halted(&self) -> bool {
        !self.is_running()
    }

    fn halt(&mut self) {
        self.set_state(StateValue::Halted);
    }

    fn resume(&mut self) {
        self.set_state(StateValue::Running);
    }
}

/// A manager for different [JsonTopicProcessor] that have been registered via `register()`.
/// Will select the appropriate processor for incoming messages on different topics.
#[derive(Default)]
pub struct TopicManager<'a> {
    processors: HashMap<String, &'a mut dyn JsonTopicProcessor>,
}

impl<'a> TopicManager<'a> {
    /// Register ´processor` for topic `topic`.
    pub fn register(&mut self, topic: impl AsRef<str>, processor: &'a mut impl JsonTopicProcessor) {
        self.processors.insert(topic.as_ref().into(), processor);
    }

    /// Halt the processor for the given `topic`. If already halted, nothing happens.
    pub fn halt(&mut self, topic: impl AsRef<str>) {
        if let Some(proc) = self.processors.get_mut(topic.as_ref()) {
            proc.halt();
        }
    }

    /// Resume the processor for the given `topic`. If already running, nothing happens.
    pub fn resume(&mut self, topic: impl AsRef<str>) {
        if let Some(proc) = self.processors.get_mut(topic.as_ref()) {
            proc.resume();
        }
    }

    /// Chooses the appropriate processor for the given topic (if there is one) and returns it's
    /// processing-result.
    pub async fn process(&self, message: JsonMessage<'a>) -> Result<Option<Vec<JsonMessage<'_>>>> {
        let topic: &str = message.topic.borrow();
        if let Some(proc) = self.processors.get(topic) {
            if proc.is_running() {
                return Ok(proc.process(message.json).await?);
            }
        }
        Ok(None)
    }
}
