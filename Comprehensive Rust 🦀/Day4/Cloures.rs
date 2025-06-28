pub fn run_closure_pipe_example() {

    let say_hello = || {
        println!("Hello, from a no-parameter closure!");
    };

    say_hello();
}

fn main() {
    run_closure_pipe_example();
}
