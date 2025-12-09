fn main() {
    let message = String::from("Single use Ticket");

    for i in 0..3 {
        println!(" Iteration is {}", i);

        consume_data(message.clone());
    }
}

fn consume_data(input: String) {
    println!(" Consumed {}", input);
}
