use card_macros::EnumToStr;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone)]
pub enum Suit {
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

#[derive(EnumToStr, EnumIter, Clone, Eq, Hash, PartialEq)]
pub enum Rank {
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
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit: suit,
            rank: rank,
        }
    }
    pub fn to_string(&self) -> String {
        format!("[{} {}]", self.rank.to_string(), self.suit.to_string(),)
    }
    pub fn to_value(&self, card_values: &HashMap<Rank, u32>) -> u32 {
        match card_values.get(&self.rank) {
            Some(value) => *value,
            None => 0,
        }
    }
}

pub struct CardStack {
    pub stack: Vec<Card>,
}

impl CardStack {
    pub fn new() -> Self {
        let mut stack = Vec::new();
        for suit_item in Suit::iter() {
            for rank_item in Rank::iter() {
                stack.push(Card::new(suit_item.clone(), rank_item.clone()));
            }
        }
        CardStack { stack: stack }
    }
}
