#[target_feature(enable = "sse2")]
unsafe fn FastCompute(x: f64) -> f64 {
    x.sqrt()
}
