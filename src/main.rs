use clap::Parser;

mod arguments;
mod deck;
mod game_manager;
mod turn_manager;

fn main() {
    let args = arguments::Main::parse();

    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("#################################");
    println!("############## StS ##############");
    let mut game_manager = game_manager::GameManager::new();

    let my_card = deck::Card {
        name: "My Card".to_string(),
        card_type: "My Type".to_string(),
    };

    game_manager.deck.add_card(my_card);
    game_manager.deck.shuffle();

    match args.command {
        arguments::Actions::Attack => game_manager.attack().unwrap(),
        arguments::Actions::Status => println!("{:#?}", game_manager.deck),
    }
}
