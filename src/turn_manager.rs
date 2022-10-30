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

    pub(crate) fn switch(mut self) {
        self.current = match self.current {
            Turn::Player => Turn::Enemy,
            Turn::Enemy => Turn::Player,
        };
    }
}
