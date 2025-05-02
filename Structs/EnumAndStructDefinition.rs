#[derive (Debug)] 

enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct  Card {
    rank: String,
    suit: CardSuit,
}

fn main() {
    let FirstCard: CardSuit = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}" , second_card);

    let CardSuit=[CardSuit::Hearts , CardSuit::Clubs];
    let CardSuit = (CardSuit::Hearts , CardSuit::Spades);
}
