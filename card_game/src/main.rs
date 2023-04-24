use cards::card::{Card, CardStack};
use std::{io, str::CharIndices};

fn main() {
    let mut sum_player: i32 = 0;
    let mut sum_bank = 0;
    let mut player_win = true;
    let mut card_stack = CardStack::new();

    println!("Player {} Bank {}", sum_player, sum_bank);
    let mut card_player = get_card(&mut card_stack);

    // player draws cards
    loop {
        println!("Card for Player {}", card_player.to_string());
        let mut answer = String::new();
        println!("Card? Y/[N]");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        answer = answer.trim().to_string();
        if answer == "Y" || answer == "y" {
            card_player = get_card(&mut card_stack);
        } else {
            break;
        }
    }
    // bank draws cards
    println!("Player win");
}

fn get_card(card_stack: &mut CardStack) -> Card {
    let mut rng = rand::thread_rng();
    card_stack.stack.pop().unwrap()
}
