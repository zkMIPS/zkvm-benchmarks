use utils::benchmark_v2;
use zkm_script::{bench_bigmem, init_logger};

fn main() {
    init_logger();

    let values = [5u32];
    benchmark_v2(bench_bigmem, &values, "../benchmark_outputs/bigmem_zkm2.csv", "value");
}
