use std::time::Duration;

use methods::{
    BIGMEM_ELF, BIGMEM_ID
};
use risc0_zkvm::{LocalProver, ExecutorEnv, Prover};
use utils::{benchmark, size};

fn main() {
    let values = [5];
    benchmark(bench_bigmem, &values, "../../benchmark_outputs/bigmem_risczero.csv", "n");
}

fn bench_bigmem(n: u32) -> (Duration, usize) {
    let env = ExecutorEnv::builder().write::<u32>(&n).unwrap().build().unwrap();
    let prover = LocalProver::new("prover");

    println!("benchmark_bigmem start, value: {}", n);
    let start = std::time::Instant::now();
    let receipt = prover.prove(env, BIGMEM_ELF).unwrap().receipt;
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_bigmem end, duration: {:?}", duration.as_secs_f64());

    let _output: u32 = receipt.journal.decode().unwrap();
    receipt.verify(BIGMEM_ID).unwrap();
    
    (duration, size(&receipt))
}
