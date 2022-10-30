use clap::Parser;

mod deck;
mod arguments;

use arguments::Args;


fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)

   }

   let mut my_deck = deck::Deck::new();
   
   let my_card = deck::Card {
       name: "My Card".to_string(),
       card_type: "My Type".to_string(),
   };

   my_deck.add_card(my_card);
   my_deck.shuffle();
   println!("{my_deck:#?}");
}
