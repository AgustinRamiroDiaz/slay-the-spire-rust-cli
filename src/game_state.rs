use serde::{Deserialize, Serialize};

// All the properties will need to derive this.
// DO NOT derive this in other structs just to make it work.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct GameState {
    pub(crate) situation: Situation,
    pub(crate) player: Player,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Enemy {
    pub(crate) health: usize,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Player {
    pub(crate) health: usize,
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Turn {
    Player,
    Enemy,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Situation {
    Fighting { enemy: Enemy, turn: Turn },
}

impl GameState {
    pub(crate) fn new() -> Self {
        Self {
            player: Player {
                health: 20,
                name: "di peq".to_string(),
            },
            situation: Situation::Fighting {
                enemy: Enemy {
                    health: 3,
                    name: "The heart".to_string(),
                },
                turn: Turn::Player,
            },
        }
    }
}
