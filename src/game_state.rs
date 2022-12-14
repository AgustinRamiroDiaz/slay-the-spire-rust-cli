use serde::{Deserialize, Serialize};

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
pub(crate) struct FightCards {
    pub(crate) draw_pile: Vec<Card>,
    pub(crate) hand: Vec<Card>,
    pub(crate) discard_pile: Vec<Card>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Fight {
    pub(crate) armor: usize,
    pub(crate) enemy: Enemy,
    pub(crate) turn: Turn,
    pub(crate) fight_cards: FightCards,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Situation {
    Fight(Fight),
    Won,
    Chill,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) enum CardContent {
    Attack(usize),
    Defend(usize),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Card {
    pub(crate) name: String,
    pub(crate) card_content: CardContent,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct GameState {
    pub(crate) situation: Situation,
    pub(crate) player: Player,
    pub(crate) deck: Vec<Card>,
}

impl GameState {
    pub(crate) fn new() -> Self {
        let mut deck = Vec::new();
        for _ in 0..5 {
            deck.push(Card {
                name: "Strike".to_string(),
                card_content: CardContent::Attack(6),
            });
            deck.push(Card {
                name: "Defend".to_string(),
                card_content: CardContent::Defend(5),
            });
        }

        Self {
            player: Player {
                health: 20,
                name: "di peq".to_string(),
            },
            deck,
            situation: Situation::Chill,
        }
    }
}
