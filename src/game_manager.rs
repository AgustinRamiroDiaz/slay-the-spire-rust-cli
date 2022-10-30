use crate::deck;
use crate::turn_manager::TurnManager;
use deck::Deck;

pub(crate) struct GameManager {
    pub(crate) deck: Deck,
    pub(crate) turn_manager: TurnManager,
}

impl GameManager {
    pub(crate) fn new() -> Self {
        GameManager {
            deck: Deck::new(),
            turn_manager: TurnManager::new(),
        }
    }
}
