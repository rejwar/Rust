use mpc_framework::MultipartyComputation;

fn SecureComputation() {
    let parties = MultipartyComputation::new(3);
    parties.run(|| {
        println!("Secure Computation Executed!");
    });
}

fn main() {
    SecureComputation();
}
