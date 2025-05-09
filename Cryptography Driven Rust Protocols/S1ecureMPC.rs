use mpc_framework::MultipartyComputation;

fn SecureComputation() {
    let parties = MultipartyComputation::new(4);
    parties.run(|| {
        println!("Secure Multi-Party Computation executed!");
    });
}

fn main() {
    SecureComputation();
}
