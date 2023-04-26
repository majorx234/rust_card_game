use cards::card::{Card, CardStack, Rank};
use std::collections::HashMap;
use std::{io, str::CharIndices};

fn main() {
    let mut values: HashMap<Rank, u32> = HashMap::new();
    values.insert(Rank::Ace, 11);
    values.insert(Rank::King, 10);
    values.insert(Rank::Queen, 10);
    values.insert(Rank::Jack, 10);
    values.insert(Rank::Ten, 10);
    values.insert(Rank::Nine, 9);
    values.insert(Rank::Eight, 8);
    values.insert(Rank::Seven, 7);
    values.insert(Rank::Six, 6);
    values.insert(Rank::Five, 5);
    values.insert(Rank::Four, 4);
    values.insert(Rank::Three, 3);
    values.insert(Rank::Two, 2);

    let mut sum_player: u32 = 0;
    let mut sum_bank = 0;
    let mut player_win = true;
    let mut card_stack = CardStack::new();

    println!("Player {} Bank {}", sum_player, sum_bank);
    let mut card_player = get_card(&mut card_stack);
    sum_player += card_player.to_value(&values);

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
        sum_player += card_player.to_value(&values);
        if sum_player > 21 {
            println!("You lose");
            return;
        }
    }
    // bank draws cards
    println!("Player win: {}", sum_player);
}

fn get_card(card_stack: &mut CardStack) -> Card {
    let mut rng = rand::thread_rng();
    card_stack.stack.pop().unwrap()
}
