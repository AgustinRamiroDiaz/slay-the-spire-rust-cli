use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct GameState {
    test: String,
}

impl GameState {
    pub(crate) fn new() -> Self {
        Self {
            test: "peke".to_string(),
        }
    }
}
