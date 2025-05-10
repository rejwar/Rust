use mpc_framework::MultipartyComputation;

pub fn run_smpc() {
    let parties = MultipartyComputation::new(4);
    parties.run(|| {
        println!("Secure Computation Executed!");
    });
}

fn main() {
    run_smpc();
}
