use winterfell::{StarkProof, StarkVerifier, TraceTable};

fn GenerateProof() -> StarkProof {
    let Data = vec![1, 2, 3, 4, 5];
    let Table = TraceTable::new(Data);
    StarkProof::generate(&Table)
}

fn main() {
    let Proof = GenerateProof();
    assert!(StarkVerifier::verify(&Proof));
    println!("zk-STARK Proof Verified!");
}
