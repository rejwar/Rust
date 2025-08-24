use syn::token::Mut;

struct Player {
    Score: i32,
    Lives: i32,
}

fn main() {
    let mut P = Player {Score: 0 , Lives: 3};

    let PRef = &mut P;
    {
        let ScoreRef = &mut PRef.Score;
        *ScoreRef += 10;
    }

    println!("Lives -> {}", PRef.Lives);
    println!("Score -> {}", PRef.Score);
}