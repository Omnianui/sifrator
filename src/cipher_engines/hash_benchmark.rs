use std::time::{Instant, Duration};

use sha2::{Sha256, Digest as d10};
use sha3::Sha3_256;
use whirlpool::Whirlpool;
use tiger::Tiger;
use ripemd::Ripemd256;

// Definované délky vstupu v bajtech
const INPUT_SIZES: [usize; 4] = [10, 20, 50, 100];
// Celková doba běhu benchmarku pro každý algoritmus
const BENCHMARK_DURATION: Duration = Duration::from_secs(20);

trait HashFunction: d10  + Clone {
    fn new() -> Self;
}

impl HashFunction for Sha256 {
    fn new() -> Self { Sha256::default() }
}
impl HashFunction for Sha3_256 {
    fn new() -> Self { Sha3_256::default() }
}
impl HashFunction for Whirlpool {
    fn new() -> Self { Whirlpool::default() }
}
impl HashFunction for Tiger {
    fn new() -> Self { Tiger::default() }
}
impl HashFunction for Ripemd256 {
    fn new() -> Self { Ripemd256::default() }
}

struct BenchmarkResult {
    function_name: String,
    hash_counts: Vec<u64>,
}

fn benchmark_hash_function<H: HashFunction>(name: &str) -> BenchmarkResult {
    println!("Benchmarking: {}...", name);
    let mut hash_counts = Vec::new();

    for &size in INPUT_SIZES.iter() {
        let input_data = vec![0u8; size]; 
        let mut count: u64 = 0;
        let start = Instant::now();
        
        while start.elapsed() < BENCHMARK_DURATION {
            let mut hasher = <H as HashFunction>::new();
            hasher.update(&input_data);
            let _result = hasher.finalize();
            count += 1;
        }

        hash_counts.push(count);
        println!("  - Délka {} bajtů: {} hashů", size, count);
    }
    
    BenchmarkResult {
        function_name: name.to_string(),
        hash_counts,
    }
}

