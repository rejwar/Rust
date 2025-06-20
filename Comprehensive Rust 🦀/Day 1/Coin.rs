enum Coin {
    Taka,
    Poisha,
    Rupee,
    Nothing,
}

fn value_in_coin (coin: Coin) -> u8 {
    match coin {
        Coin::Taka =>100,
        Coin::Nothing => 00,
        Coin::Poisha =>01,
        Coin::Rupee => 50,
    }
}
