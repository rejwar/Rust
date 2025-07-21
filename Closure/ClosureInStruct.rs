struct Action {
    run: Box<dyn Fn()>,
}

fn main() {
    let act = Action {
        run: Box::new(|| println!("Runnig from struct")),

    };

    (act.run)();
}