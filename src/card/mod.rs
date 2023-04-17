enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

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
