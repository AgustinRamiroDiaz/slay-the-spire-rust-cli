use clap::Parser;

mod arguments;
mod deck;
mod game_manager;
mod turn_manager;

use arguments::Args;

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

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
    println!("{:#?}", game_manager.deck);
}
