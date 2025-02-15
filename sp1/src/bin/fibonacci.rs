use utils::benchmark;
use sp1_script::{bench_fibonacci, init_logger};

fn main() {
    init_logger();

    let ns = [100, 1000, 10000, 50000];
    benchmark(bench_fibonacci, &ns, "../benchmark_outputs/fibonacci_sp1.csv", "n");
}
