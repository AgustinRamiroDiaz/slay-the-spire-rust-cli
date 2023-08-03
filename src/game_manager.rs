use crate::deck;
use crate::game_state::{self, Enemy};

#[derive(Debug)]
enum Event {
    EnemyDied,
}

#[derive(Debug)]
pub(crate) struct GameManager<'a> {
    pub(crate) state: &'a mut game_state::GameState,
    event_store: Vec<Event>,
}

impl<'a> GameManager<'a> {
    pub(crate) fn new(game_state: &'a mut game_state::GameState) -> Self {
        GameManager {
            state: game_state,
            event_store: Vec::new(),
        }
    }

    pub(crate) fn enter_fight(&mut self) -> Result<(), String> {
        match self.state.situation {
            game_state::Situation::Chill => {
                self.state.situation = game_state::Situation::Fight(game_state::Fight {
                    armor: 0,
                    enemy: Enemy {
                        health: 20,
                        name: "The heart".to_string(),
                    },
                    turn: game_state::Turn::Player,
                    fight_cards: deck::new(self.state.deck.clone()),
                });
                Ok(())
            }
            _ => Err("Not Chilling".to_string()),
        }
    }

    pub(crate) fn play(&mut self, card_index: usize) -> Result<(), String> {
        match &mut self.state.situation {
            game_state::Situation::Fight(fight) => match fight.turn {
                game_state::Turn::Enemy => Err("Not your turn".to_string()),
                game_state::Turn::Player => {
                    match fight.fight_cards.hand.get(card_index) {
                        Some(card) => {
                            match &card.card_content {
                                game_state::CardContent::Attack(damage) => {
                                    attack_enemy(*damage, &mut fight.enemy, &mut self.event_store)
                                }
                                game_state::CardContent::Defend(armor) => {
                                    fight.armor += armor;
                                    println!(
                                        "You've gained {} armor, you now have {}",
                                        armor, fight.armor
                                    );
                                }
                            }
                            fight
                                .fight_cards
                                .discard_pile
                                .push(fight.fight_cards.hand.remove(card_index));
                            Ok(())
                        }
                        None => Err("No card at that index".to_string()),
                    }?;
                    Ok(())
                }
            },
            game_state::Situation::Won => Err("You won".to_string()),
            game_state::Situation::Chill => Err("You are chilling".to_string()),
        }?;

        for event in &self.event_store {
            match event {
                Event::EnemyDied => {
                    println!("You won");
                    self.state.situation = game_state::Situation::Won;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn end_turn(&mut self) -> Result<(), String> {
        match &mut self.state.situation {
            game_state::Situation::Fight(fight) => match fight.turn {
                game_state::Turn::Enemy => Err("Not your turn".to_string()),
                game_state::Turn::Player => {
                    deck::discard_hand(
                        &mut fight.fight_cards.hand,
                        &mut fight.fight_cards.discard_pile,
                    );
                    switch_turn(&mut fight.turn);
                    play_enemy(&mut fight.enemy, &mut self.state.player);
                    switch_turn(&mut fight.turn);
                    deck::draw_n(
                        5,
                        &mut fight.fight_cards.draw_pile,
                        &mut fight.fight_cards.hand,
                        &mut fight.fight_cards.discard_pile,
                    );
                    Ok(())
                }
            },
            _ => Err("Not in a fight".to_string()),
        }
    }

    pub(crate) fn peek_hand(&self) -> Result<Vec<game_state::Card>, String> {
        match &self.state.situation {
            game_state::Situation::Fight(fight) => Ok(fight.fight_cards.hand.clone()),
            _ => Err("Not in a fight".to_string()),
        }
    }

    pub(crate) fn peek_draw_pile(&self) -> Result<&Vec<game_state::Card>, String> {
        match &self.state.situation {
            game_state::Situation::Fight(fight) => Ok(&fight.fight_cards.draw_pile),
            _ => Err("Not in a fight".to_string()),
        }
    }

    pub(crate) fn peek_discard_pile(&self) -> Result<&Vec<game_state::Card>, String> {
        match &self.state.situation {
            game_state::Situation::Fight(fight) => Ok(&fight.fight_cards.discard_pile),
            _ => Err("Not in a fight".to_string()),
        }
    }

    pub(crate) fn peek_enemy(&self) -> Result<&Enemy, String> {
        match &self.state.situation {
            game_state::Situation::Fight(fight) => Ok(&fight.enemy),
            _ => Err("Not in a fight".to_string()),
        }
    }
}

fn attack_enemy(damage: usize, enemy: &mut game_state::Enemy, event_store: &mut Vec<Event>) {
    println!("You attack the enemy for {damage} damage");
    enemy_recieve_damage(damage, enemy, event_store);
}

fn enemy_recieve_damage(
    damage: usize,
    enemy: &mut game_state::Enemy,
    event_store: &mut Vec<Event>,
) {
    if enemy.health > damage {
        enemy.health -= damage;
        println!(
            "{enemy_name} has {health} health left",
            enemy_name = enemy.name,
            health = enemy.health
        );
    } else {
        println!("{} has died", enemy.name);
        enemy.health = 0;
        event_store.push(Event::EnemyDied);
    }
}

fn switch_turn(turn: &mut game_state::Turn) {
    *turn = match turn {
        game_state::Turn::Player => game_state::Turn::Enemy,
        game_state::Turn::Enemy => game_state::Turn::Player,
    };
}

fn play_enemy(_enemy: &mut game_state::Enemy, player: &mut game_state::Player) {
    println!("The heart beats faster.");
    let damage = 1;
    if player.health > damage {
        player.health -= damage;
        println!(
            "{name} has {health} health left",
            name = player.name,
            health = player.health
        );
    } else {
        println!("{} has died", player.name);
        player.health = 0;
    }
}

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn _t() {}
}
