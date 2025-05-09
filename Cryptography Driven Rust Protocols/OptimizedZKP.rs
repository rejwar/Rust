use halo2_proofs::{plonk, circuit::SimpleFloorPlanner};

struct ZKPExample {
    pub value: u32,
}

impl plonk::Circuit<u32> for ZKPExample {
    fn configure(meta: &mut plonk::ConstraintSystem<u32>) -> plonk::Config {
        plonk::Config::default()
    }

    fn synthesize(
        &self,
        config: plonk::Config,
        layouter: &mut impl plonk::Layouter<u32>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(|| "Computing ZKP", |mut region| {
            region.assign_advice(|| "Input", config.advice[0], 0, || Ok(self.value))
        })
    }
}

fn main() {
    println!("Optimized zk-SNARK computation in Rust!");
}
