use card_macros::EnumToStr;

enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl Suit {
    fn to_string(&self) -> String {
        match *self {
            Suit::Clubs => "x".to_string(),
            Suit::Diamonds => "<>".to_string(),
            Suit::Hearts => "<3".to_string(),
            Suit::Spades => "->".to_string(),
        }
    }
}

#[derive(EnumToStr)]
enum Rank {
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Ace,
}

pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn to_string(&self) {
        format!("[{} {}]", self.rank.to_string(), self.suit.to_string(),);
    }
}
