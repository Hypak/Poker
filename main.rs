#![allow(dead_code)]
use enum_iterator::{all, Sequence};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Hearts => write!(f, "H"),
            Self::Diamonds => write!(f, "D"),
            Self::Clubs => write!(f, "C"),
            Self::Spades => write!(f, "S"),
        }
    }
}
/*
impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as isize).cmp(&(*other as isize))
    }
}
*/
#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "T"),
            Self::Jack => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as isize).cmp(&(*other as isize))
    }
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.rank)?;
        write!(f, "{}", self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = vec![];
        // Iterates over all possible enum types i.e. all cards
        for card in all::<Card>() {
            cards.push(card);
        }
        return Deck { cards };
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    fn draw_many(&mut self, count: usize) -> Vec<Card> {
        let mut cards = vec![];
        for _ in 0..count {
            match self.draw() {
                Some(card) => cards.push(card),
                None => break,
            }
        }
        return cards;
    }
}

//I assume that the dervied Ord reflects the Type order, and the order of the Rank within a type
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair(Rank),
    TwoPair(Rank, Rank),
    Trip(Rank),
    Straight,
    Flush,
    FullHouse(Rank, Rank),
    Quad(Rank),
    StraightFlush,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HighCard => write!(f, "High Card"),
            Self::OnePair(rank) => write!(f, "Dub of {}", rank),
            Self::TwoPair(rank1, rank2) => write!(f, "Dubs of {} and {}", rank1, rank2),
            Self::Trip(rank) => write!(f, "Trip of {}", rank),
            Self::Straight => write!(f, "Straight"),
            Self::Flush => write!(f, "Flush"),
            Self::FullHouse(rank1, rank2) => write!(f, "Full House of {} over {}", rank1, rank2),
            Self::Quad(rank) => write!(f, "Quad of {}", rank),
            Self::StraightFlush => write!(f, "Straight Flush"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct FiveCardHand {
    cards: BTreeSet<Card>,
}

impl FiveCardHand {
    fn new(hand: Vec<Card>) -> Self {
        if hand.len() != 5 {
            panic!("Hand size should be five");
        }
        let cards: BTreeSet<Card> = hand.into_iter().collect();
        FiveCardHand { cards }
    }
    fn draw_from(deck: &mut Deck) -> Option<Self> {
        let mut cards = vec![];
        for _ in 0..5 {
            match deck.draw() {
                Some(card) => cards.push(card),
                None => return None,
            }
        }
        Some(FiveCardHand::new(cards))
    }
    fn is_straight(&self) -> bool {
        let mut old_rank = self.cards.first().expect("Five cards so not-empty").rank;
        for card in self.cards.iter().skip(1) {
            if (card.rank as isize) != (old_rank as isize) + 1 {
                if card.rank != Rank::Ace || old_rank == Rank::Five {
                    // A 2 3 4 5
                    return false;
                }
            }
            old_rank = card.rank;
        }
        return true;
    }
    fn is_flush(&self) -> bool {
        let suit = self.cards.first().expect("Five cards so not-empty").suit;
        for card in self.cards.iter().skip(1) {
            if card.suit != suit {
                return false;
            }
        }
        return true;
    }
    fn get_rank_from_index(index: usize) -> Option<Rank> {
        match index {
            0 => Some(Rank::Two),
            1 => Some(Rank::Three),
            2 => Some(Rank::Four),
            3 => Some(Rank::Five),
            4 => Some(Rank::Six),
            5 => Some(Rank::Seven),
            6 => Some(Rank::Eight),
            7 => Some(Rank::Nine),
            8 => Some(Rank::Ten),
            9 => Some(Rank::Jack),
            10 => Some(Rank::Queen),
            11 => Some(Rank::King),
            12 => Some(Rank::Ace),
            _ => None,
        }
    }
    // If there are duplicate ranks, this returns the HandType
    // Otherwise it returns None
    // If there are duplicates there is neither straight nor flush
    fn get_dist_hand_type(&self) -> Option<HandType> {
        // How many cards of each rank there are
        // 'dist' is short for 'distribution'
        let mut dist = [0; 13];
        for card in self.cards.iter() {
            let index: usize = (card.rank as isize)
                .try_into()
                .expect("the enum should be using unsigned ints");
            dist[index] += 1;
        }
        let quad_index = dist.iter().position(|&x| x == 4);
        if let Some(index) = quad_index {
            return Some(HandType::Quad(
                FiveCardHand::get_rank_from_index(index).expect("valid range"),
            ));
        }
        let triple_index = dist.iter().position(|&x| x == 3);
        let pair_indicies: Vec<usize> = dist.iter().positions(|&x| x == 2).collect();
        match triple_index {
            Some(i) => {
                let rank = FiveCardHand::get_rank_from_index(i).expect("valid range");
                match pair_indicies.len() > 0 {
                    true => {
                        let pair_rank = FiveCardHand::get_rank_from_index(pair_indicies[0])
                            .expect("valid range");
                        return Some(HandType::FullHouse(rank, pair_rank));
                    }
                    false => return Some(HandType::Trip(rank)),
                }
            }
            None => match pair_indicies.len() {
                // Let's hope there aren't any 3-pairs
                2 => {
                    let rank_0 =
                        FiveCardHand::get_rank_from_index(pair_indicies[0]).expect("valid range");
                    let rank_1 =
                        FiveCardHand::get_rank_from_index(pair_indicies[1]).expect("valid range");
                    return Some(HandType::TwoPair(rank_1, rank_0));
                }
                1 => {
                    let rank =
                        FiveCardHand::get_rank_from_index(pair_indicies[0]).expect("valid range");
                    return Some(HandType::OnePair(rank));
                }
                _ => return None,
            },
        }
    }
    fn get_hand_type(&self) -> HandType {
        let dist_hand_type = self.get_dist_hand_type();
        match dist_hand_type {
            Some(res) => res,
            None => match self.is_straight() {
                true => match self.is_flush() {
                    true => return HandType::StraightFlush,
                    false => return HandType::Straight,
                },
                false => match self.is_flush() {
                    true => return HandType::Flush,
                    false => return HandType::HighCard,
                },
            },
        }
    }
    fn get_best_hand(cards: impl IntoIterator<Item = Card>) -> Self {
        let combinations: Vec<Vec<Card>> = cards.into_iter().combinations(5).collect();
        let mut hands = vec![];
        for hand in combinations.iter() {
            hands.push(FiveCardHand::new(hand.to_vec()));
        }
        return hands.iter().max().expect("pls").clone();
    }
}

impl fmt::Display for FiveCardHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for card in &self.cards {
            write!(f, "{} ", card)?;
        }
        write!(f, "]")
    }
}

impl PartialOrd for FiveCardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for FiveCardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_hand_type();
        let other_type = other.get_hand_type();
        match self_type.cmp(&other_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let mut self_iter = self.cards.iter();
                let mut other_iter = other.cards.iter();
                // Check the rank of the cards
                for _ in (0..5).rev() {
                    match self_iter.next().cmp(&other_iter.next()) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {} // Continue to next loop
                    }
                }
                Ordering::Equal
            }
        }
    }
}

/*
fn simulate_random(hand: (Card, Card), table: Vec<Card>, other_count: usize) -> Ordering {
    let mut deck_left = Deck::new();
    deck_left.cards.remove(hand.0);
    todo!();
}
*/

fn print_cards(cards: &Vec<Card>) {
    print!("[");
    for card in cards {
        print!("{} ", card);
    }
    print!("]");
}

fn main2() {
    let deck = Deck::new().cards;
    let hand_a = vec![deck[7], deck[20], deck[17], deck[2], deck[21]];
    let hand_b = vec![deck[6], deck[19], deck[16], deck[14], deck[22]];
    print_cards(&hand_a);
    println!();
    print_cards(&hand_b);
    println!();
    println!("{}", FiveCardHand::new(hand_a) > FiveCardHand::new(hand_b));
}

fn round() -> isize {
    let mut deck = Deck::new();
    deck.shuffle();
    let mut hand_a = deck.draw_many(2);
    let mut hand_b = deck.draw_many(2);

    let table = deck.draw_many(5);
    hand_a.extend(&table);
    hand_b.extend(&table);
    let a_best = FiveCardHand::get_best_hand(hand_a.clone());
    let b_best = FiveCardHand::get_best_hand(hand_b.clone());

    if a_best.get_hand_type() < HandType::FullHouse(Rank::Two, Rank::Three) {
        return 0;
    }

    print_cards(&hand_a);
    print!(" ");
    print_cards(&hand_b);
    println!();
    print!("Table: ");
    print_cards(&table);
    println!();
    println!("{} \tvs {}", a_best, b_best);
    println!(
        "{}\t\tvs {}",
        a_best.get_hand_type(),
        b_best.get_hand_type()
    );
    match a_best.cmp(&b_best) {
        Ordering::Greater => println!("Winner\t\t\tvs Loser"),
        Ordering::Less => println!("Loser\t\t\tvs Winner"),
        Ordering::Equal => println!("It's a draw!?"),
    }
    return 1;
}

fn main() {
    let mut total = 0;
    let mut count = 0;
    loop {
        count += round();
        total += 1;
        if total % 1_000 == 0 {
            println!();
            println!("{count} / {total}");
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(HandType::OnePair(Rank::King) > HandType::OnePair(Rank::Queen));
    }
}
