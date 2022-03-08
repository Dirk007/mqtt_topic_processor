pub mod implementations;
pub mod json_message;
pub mod processor;
pub mod state;

pub use json_message::{JsonDeserializable, JsonMessage};
pub use processor::{JsonTopicProcessor, TopicManager};
pub use state::{SimpleState, StateValue};

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[tokio::test]
    async fn test() -> Result<()> {
        use crate::{implementations::Forwarder, TopicManager};

        let mut manager = TopicManager::default();
        let forwarder = Box::new(Forwarder::new("other/topic".into()));
        manager.register("foo/bar", forwarder);

        // Unhandled Topic -> None
        assert_eq!(
            manager.process(("foo/blubb", r#"{"hello":"world"}"#).into()).await?,
            None
        );

        // Handled Topic -> Processed Result == new topic
        assert_eq!(
            manager.process(("foo/bar", r#"{"hello":"world"}"#).into()).await?,
            Some(vec![("other/topic", r#"{"hello":"world"}"#).into()])
        );

        // Do not processs when halted
        manager.halt("foo/bar");
        assert_eq!(manager.process(("foo/bar", r#"{"hello":"world"}"#).into()).await?, None);

        // And resume when resumed
        manager.resume("foo/bar");
        assert_eq!(
            manager.process(("foo/bar", r#"{"hello":"world"}"#).into()).await?,
            Some(vec![("other/topic", r#"{"hello":"world"}"#).into()])
        );

        Ok(())
    }
}
