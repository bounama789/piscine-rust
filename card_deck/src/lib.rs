use rand::{thread_rng, Rng};

#[derive(PartialEq,Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(PartialEq,Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let r = thread_rng().gen_range(1, 4);

        Suit::translate(r)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => todo!()
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let r = thread_rng().gen_range(1, 13);

        Rank::translate(r)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Number(value),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace && card.suit == Suit::Spade
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };
    	println!("Your card is {:?}", your_card);

        let result = winner_card(&your_card);
        assert_eq!(result, your_card.rank == Rank::Ace && your_card.suit == Suit::Spade);
    }
}
