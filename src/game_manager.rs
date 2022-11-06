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

    pub(crate) fn attack(&mut self) -> Result<(), String> {
        match &mut self.state.situation {
            game_state::Situation::Fighting { enemy, turn } => match turn {
                game_state::Turn::Enemy => Err("Not your turn".to_string()),
                game_state::Turn::Player => {
                    attack_enemy(1, enemy, &mut self.event_store);
                    switch_turn(turn);
                    play_enemy(enemy, &mut self.state.player);
                    switch_turn(turn);
                    Ok(())
                }
            },
            game_state::Situation::Won => Err("You won".to_string()),
        }?;

        for event in &self.event_store {
            match event {
                Event::EnemyDied => {
                    println!("Enemy died");
                    println!("You won");
                    self.state.situation = game_state::Situation::Won;
                }
            }
        }
        Ok(())
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

fn play_enemy(enemy: &mut game_state::Enemy, player: &mut game_state::Player) {
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
