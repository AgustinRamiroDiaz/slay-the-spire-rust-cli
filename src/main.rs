use std::error::Error;

use clap::Parser;

mod arguments;
mod deck;
mod game_manager;
mod game_state;
mod loader;

fn main() -> Result<(), Box<dyn Error>> {
    let args = arguments::Main::parse();

    let mut loader = loader::Loader::new()?;
    match &args.command {
        arguments::Actions::Save(save_command) => match save_command {
            arguments::Save::Delete => loader.delete(),
            arguments::Save::Load(save) => loader = loader.with_file(save.file.clone())?,
        },
        _ => (),
    }
    let mut game_state = loader.load()?;

    let mut game_manager = game_manager::GameManager::new(game_state);

    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("############## StS ##############");

    match &args.command {
        arguments::Actions::Save(_) => {}
        arguments::Actions::Play(card_index) => game_manager.play(card_index.index)?,
        arguments::Actions::Peek(game_object) => match game_object {
            arguments::GameObject::Player => println!("{:#?}", game_manager.state.get_player()),
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
    loader.save(&game_manager.state)?;
    Ok(())
}
