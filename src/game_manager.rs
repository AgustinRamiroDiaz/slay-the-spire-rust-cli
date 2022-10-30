use crate::deck;
use crate::turn_manager;
use deck::Deck;

pub(crate) struct GameManager {
    pub(crate) deck: Deck,
    pub(crate) turn_manager: turn_manager::TurnManager,
    pub(crate) state: State,
}

type Enemy = String;
type Rewards = String;

pub(crate) enum State {
    Fight(Enemy),
    PickingRewards(Rewards),
}

impl GameManager {
    pub(crate) fn new() -> Self {
        GameManager {
            deck: Deck::new(),
            turn_manager: turn_manager::TurnManager::new(),
            state: State::Fight("the heart".to_string()),
        }
    }

    pub(crate) fn attack(self) -> Result<(), String> {
        match self.turn_manager.current {
            turn_manager::Turn::Enemy => Err("Not your turn".to_string()),
            turn_manager::Turn::Player => {
                println!("you missed");
                Ok(())
            }
        }
    }
}
