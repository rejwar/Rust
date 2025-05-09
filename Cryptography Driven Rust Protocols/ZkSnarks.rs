use bellman::{groth16, Circuit};
use bls12_381::Bls12;

struct ExampleCircuit {
    pub a: i32,
    pub b: i32,
}

impl Circuit<Bls12> for ExampleCircuit {
    fn synthesize<CS: ConstraintSystem<Bls12>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let A = cs.alloc(|| "a", || Ok(self.a.into()))?;
        let B = cs.alloc(|| "b", || Ok(self.b.into()))?;
        cs.enforce(|| "Multiplication Constraint", |lc| lc + A, |lc| lc + B, |lc| lc);
        Ok(())
    }
}
