use crate::deck;
use crate::game_state;
use crate::turn_manager;
use deck::Deck;

pub(crate) struct GameManager {
    pub(crate) deck: Deck,
    pub(crate) turn_manager: turn_manager::TurnManager,
    pub(crate) now_doing: NowDoing,
    pub(crate) state: game_state::GameState,
}

type Enemy = String;
type Rewards = String;

pub(crate) enum NowDoing {
    Fight(Enemy),
    PickingRewards(Rewards),
}

impl GameManager {
    pub(crate) fn new(game_state: game_state::GameState) -> Self {
        GameManager {
            deck: Deck::new(),
            turn_manager: turn_manager::TurnManager::new(),
            now_doing: NowDoing::Fight("the heart".to_string()),
            state: game_state,
        }
    }

    pub(crate) fn attack(&self) -> Result<(), String> {
        match self.turn_manager.current {
            turn_manager::Turn::Enemy => Err("Not your turn".to_string()),
            turn_manager::Turn::Player => {
                println!("you missed");
                Ok(())
            }
        }
    }
}
