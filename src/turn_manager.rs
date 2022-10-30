enum Turn {
    Player,
    Enemy,
}

pub(crate) struct TurnManager {
    current: Turn,
}

impl TurnManager {
    pub(crate) fn new() -> Self {
        Self {
            current: Turn::Player,
        }
    }
}
