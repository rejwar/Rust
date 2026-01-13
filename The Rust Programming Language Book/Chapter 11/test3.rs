#[test]
#[should_panic(expected = " Guess value must be less than or equal to 100")]

fn greater_than_100() {
    Guess::new(200);
}
