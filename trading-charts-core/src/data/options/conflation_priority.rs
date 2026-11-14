use serde::{Deserialize, Serialize};

/// Priority of the background task which precomputes the conflated data.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConflationPriority {
    #[default]
    Background,
    UserVisible,
    UserBlocking,
}
