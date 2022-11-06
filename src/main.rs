use clap::Parser;

mod arguments;
mod deck;
mod game_manager;
mod game_state;
mod loader;

fn main() {
    let args = arguments::Main::parse();

    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("############## StS ##############");
    let loader = loader::Loader::new();
    let mut game_state = loader.load();

    let mut game_manager = game_manager::GameManager::new(&mut game_state);

    match args.command {
        arguments::Actions::Attack => game_manager.attack().unwrap(),
        arguments::Actions::Status => println!("{:#?}", game_manager.state),
    }

    loader.save(&game_manager.state)
}
