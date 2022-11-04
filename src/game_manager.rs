use crate::game_state;

#[derive(Debug)]
pub(crate) struct GameManager<'a> {
    pub(crate) state: &'a mut game_state::GameState,
}

impl<'a> GameManager<'a> {
    pub(crate) fn new(game_state: &'a mut game_state::GameState) -> Self {
        GameManager { state: game_state }
    }

    pub(crate) fn attack(&mut self) -> Result<(), String> {
        match &mut self.state.situation {
            game_state::Situation::Fighting { enemy, turn } => match turn {
                game_state::Turn::Enemy => Err("Not your turn".to_string()),
                game_state::Turn::Player => {
                    attack_enemy(1, enemy);
                    switch_turn(turn);
                    play_enemy(enemy);
                    switch_turn(turn);
                    Ok(())
                }
            },
        }
    }
}

fn attack_enemy(damage: usize, enemy: &mut game_state::Enemy) {
    println!("You attack the enemy for {damage} damage");
    enemy_recieve_damage(damage, enemy)
}

fn enemy_recieve_damage(damage: usize, enemy: &mut game_state::Enemy) {
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
    }
}

fn switch_turn(turn: &mut game_state::Turn) {
    *turn = match turn {
        game_state::Turn::Player => game_state::Turn::Enemy,
        game_state::Turn::Enemy => game_state::Turn::Player,
    };
}

fn play_enemy(enemy: &mut game_state::Enemy) {
    println!("The heart beats faster.");
}
