use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::game_state::{Card, FightCards};

pub(crate) fn new(deck: Vec<Card>) -> FightCards {
    let mut deck = FightCards {
        draw_pile: deck,
        hand: Vec::new(),
        discard_pile: Vec::new(),
    };

    shuffle(&mut deck.draw_pile);

    draw_n(
        5,
        &mut deck.draw_pile,
        &mut deck.hand,
        &mut deck.discard_pile,
    );

    deck
}

fn draw_n(n: usize, draw: &mut Vec<Card>, hand: &mut Vec<Card>, discard: &mut Vec<Card>) {
    for _ in 0..n {
        draw_card(draw, hand, discard);
    }
}

fn draw_card(draw: &mut Vec<Card>, hand: &mut Vec<Card>, discard: &mut Vec<Card>) {
    match draw.pop() {
        Some(card) => hand.push(card),
        None => match discard.len() {
            0 => {}
            _ => {
                shuffle(discard);
                draw.append(discard);
                match draw.pop() {
                    Some(card) => hand.push(card),
                    None => {}
                }
            }
        },
    }
}

fn shuffle<T>(vec: &mut Vec<T>) {
    vec.shuffle(&mut thread_rng());
}
