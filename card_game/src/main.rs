use cards::card::{Card, CardStack, Rank};
use rand::Rng;
use std::collections::HashMap;
use std::{io, str::CharIndices};

fn main() {
    let mut rng = rand::thread_rng();
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
    let mut sum_bank: u32 = 0;
    let mut player_win = true;
    let mut player_quit = false;
    let mut card_stack = CardStack::new();
    card_stack.shuffle(&mut rng);

    println!("Player {} Bank {}", sum_player, sum_bank);
    let mut card_player = get_card(&mut card_stack);
    let mut card_bank = get_card(&mut card_stack);
    sum_player += card_player.to_value(&values);
    sum_bank += card_bank.to_value(&values);
    println!("Card for Player {}", card_player.to_string());
    println!("Card for Bank {}", card_bank.to_string());
    // player draws cards
    loop {
        let mut answer = String::new();
        println!("Card? Y/[N]");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        answer = answer.trim().to_string();
        if answer == "Y" || answer == "y" {
            card_player = get_card(&mut card_stack);
        } else {
            player_quit = true;
        }
        println!("Card for Player {}", card_player.to_string());
        sum_player += card_player.to_value(&values);
        if sum_player > 21 {
            println!("You lose: {}", sum_player);
            return;
        }
        if sum_player > sum_bank {
            card_bank = get_card(&mut card_stack);
            println!("Card for Bank {}", card_bank.to_string());
            sum_bank += card_bank.to_value(&values);
            if sum_bank > 21 {
                println!("Player win: {} vs {}", sum_player, sum_bank);
                return;
            }
        }
        if player_quit {
            break;
        }
        println!("Player {} Bank {}", sum_player, sum_bank);
    }
    if sum_player > sum_bank {
        println!("Player win: {} vs {}", sum_player, sum_bank);
    }
}

fn get_card(card_stack: &mut CardStack) -> Card {
    card_stack.stack.pop().unwrap()
}
