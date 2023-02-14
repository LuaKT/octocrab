use crate::models::repos::Release;
use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::ReleaseEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReleaseEventPayload {
    /// The action this event represents.
    pub action: ReleaseEventAction,
    /// The release this event corresponds to.
    pub release: Release,
    /// The changes to body or name if this event is of type [`ReleaseEventAction::Edited`].
    pub changes: Option<ReleaseEventChanges>,
}

/// The change which occurred in an event of type [`ReleaseEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ReleaseEventChanges {
    Body(ReleaseEventChangesFrom),
    Name(ReleaseEventChangesFrom),
}

/// The previous value of the item (either the body or title) of a release which has changed. Only
/// available in an event of type [`ReleaseEventAction::Edited`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReleaseEventChangesFrom {
    pub from: String,
}

/// The action on a release this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ReleaseEventAction {
    Published,
    Edited,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::events::{payload::EventPayload, Event};
    use serde_json::json;

    #[test]
    fn should_deserialize_action_from_lowercase() {
        let actions = vec![
            (r#""published""#, ReleaseEventAction::Published),
            (r#""edited""#, ReleaseEventAction::Edited),
        ];
        for (action_str, action) in actions {
            let deserialized = serde_json::from_str(&action_str).unwrap();
            assert_eq!(action, deserialized);
        }
    }

    #[test]
    fn should_deserialize_name_changes() {
        let json = json!({
            "name": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<ReleaseEventChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            ReleaseEventChanges::Name(ReleaseEventChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_body_changes() {
        let json = json!({
            "body": {
                "from": "test"
            }
        });
        let deserialized = serde_json::from_value::<ReleaseEventChanges>(json).unwrap();
        assert_eq!(
            deserialized,
            ReleaseEventChanges::Body(ReleaseEventChangesFrom {
                from: "test".to_owned()
            })
        );
    }

    #[test]
    fn should_deserialize_with_correct_payload() {
        let json = include_str!("../../../../tests/resources/release_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        if let Some(EventPayload::ReleaseEvent(payload)) = event.payload {
            assert_eq!(payload.action, ReleaseEventAction::Published);
            assert_eq!(payload.release.id.0, 92281831);
        } else {
            panic!("unexpected event payload encountered: {:#?}", event.payload);
        }
    }
}
