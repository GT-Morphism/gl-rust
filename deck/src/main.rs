use rand::{seq::SliceRandom, thread_rng};

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        const SUITS: [&str; 4] = ["Hearts", "Spades", "Clubs", "Diamonds"];
        const VALUES: [&str; 3] = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in SUITS {
            for value in VALUES {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    println!("Deck before dealing cards: {:#?}", deck.cards);
    let cards = deck.deal(3);
    println!("Deck after dealing cards: {:#?}", deck.cards);

    println!("Your cards: {:#?}", cards);
}
