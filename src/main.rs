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
        arguments::Actions::Attack => game_manager.attack().map_err(|e| e)?,
        arguments::Actions::Status => println!("{:#?}", game_manager.state),
    }

    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("#################################");
    loader.save(&game_manager.state);
    Ok(())
}
