use rand::Rng;
use std::io;

fn main() {
    let mut sum_player: i32 = 0;
    let mut sum_bank = 0;
    let mut player_win = true;

    println!("Player {} Bank {}", sum_player, sum_bank);
    let mut card_player = random_card();

    // player draws cards
    loop {
        println!("Card for Player {}", card_player);
        sum_player += card_player;
        println!("Player {} Bank {}", sum_player, sum_bank);
        let mut answer = String::new();
        println!("Card? Y/[N]");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        answer = answer.trim().to_string();
        if answer == "Y" || answer == "y" {
            card_player = random_card();
        } else {
            break;
        }
    }
    // bank draws cards
    println!("Player win");
}

fn random_card() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=11)
}
