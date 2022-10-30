use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub(crate) struct Deck {
    pub(crate) cards: Vec<Card>,
}

type Type = String;

#[derive(Debug)]
pub(crate) struct Card {
    pub(crate) name : String,
    pub(crate) card_type : Type,   
}

impl Deck {
    pub(crate) fn new() -> Deck {
        Deck {
            cards: Vec::new(),
        }
    }

    pub(crate) fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub(crate) fn remove_card(&mut self, card: Card) -> Result<(), String> {
        let position = self.cards.iter().position(|x| x.name == card.name).ok_or("Not found".to_string())?;
        self.cards.remove(position);
        Ok(())
    }

    pub(crate) fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub(crate) fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
