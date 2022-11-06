use clap::Parser;

mod arguments;
mod deck;
mod game_manager;
mod game_state;
mod loader;

fn main() -> Result<(), String> {
    let args = arguments::Main::parse();

    let loader = loader::Loader::new();
    let mut game_state = loader.load();

    let mut game_manager = game_manager::GameManager::new(&mut game_state);

    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("############## StS ##############");

    match args.command {
        arguments::Actions::Play(card_index) => game_manager.play(card_index.index)?,
        arguments::Actions::Status(game_object) => match game_object {
            arguments::GameObject::Player => println!("{:#?}", game_manager.state.player),
            arguments::GameObject::Enemy => println!("{:#?}", game_manager.peek_enemy()),
            arguments::GameObject::DrawPile => println!("{:#?}", game_manager.peek_draw_pile()?),
            arguments::GameObject::Hand => println!("{:#?}", game_manager.peek_hand()),
            arguments::GameObject::DiscardPile => {
                println!("{:?}", game_manager.peek_discard_pile())
            }
        },
        arguments::Actions::EnterFight => game_manager.enter_fight()?,
        arguments::Actions::EndTurn => game_manager.end_turn()?,
    }

    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("#################################");
    loader.save(&game_manager.state);
    Ok(())
}
