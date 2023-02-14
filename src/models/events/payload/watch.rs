use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::WatchEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WatchEventPayload {
    /// The action this event represents.
    pub action: WatchEventAction,
}

/// The action on a watch event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum WatchEventAction {
    Started,
}

#[cfg(test)]
mod test {
    use super::WatchEventAction;
    use crate::models::events::{payload::EventPayload, Event};

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/watch_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::WatchEvent(payload)) = event.payload {
            assert_eq!(payload.action, WatchEventAction::Started);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
