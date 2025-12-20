use std::time::{Instant, Duration};
use rust_xlsxwriter::{Workbook, XlsxError};

use sha2::{Sha256, Digest};
use sha3::Sha3_256;
use whirlpool::Whirlpool;
use tiger::Tiger;
use ripemd::Ripemd256;

// Definované délky vstupu v bajtech
const INPUT_SIZES: [i32; 4] = [10, 20, 50, 100];
const BENCHMARK_DURATION: Duration = Duration::from_secs(20);

trait HashFunction: Digest  + Clone {
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
        let input_data = vec![0u8; size as usize]; 
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

pub fn benchmark_console(){
    println!("--- Rychlostní srovnání hašovacích funkcí (limit: {}s) ---", BENCHMARK_DURATION.as_secs());
    
    let mut results: Vec<BenchmarkResult> = Vec::new();

    results.push(benchmark_hash_function::<Sha256>("SHA-256"));
    results.push(benchmark_hash_function::<Sha3_256>("SHA3-256"));
    results.push(benchmark_hash_function::<Whirlpool>("Whirlpool"));
    results.push(benchmark_hash_function::<Tiger>("Tiger"));
    results.push(benchmark_hash_function::<Ripemd256>("RIPEMD-256"));

    println!("\nBenchmark dokončen.");
}

pub fn benchmark_xls() -> Result<(), XlsxError>{
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let mut i = 1;
    let mut results: Vec<BenchmarkResult> = Vec::new();

    println!("--- Rychlostní srovnání hašovacích funkcí (limit: {}s) ---", BENCHMARK_DURATION.as_secs());

    results.push(benchmark_hash_function::<Sha256>("SHA-256"));
    results.push(benchmark_hash_function::<Sha3_256>("SHA3-256"));
    results.push(benchmark_hash_function::<Whirlpool>("Whirlpool"));
    results.push(benchmark_hash_function::<Tiger>("Tiger"));
    results.push(benchmark_hash_function::<Ripemd256>("RIPEMD-256"));

    worksheet.write(0, 0, "Hashovací funkce")?;
    worksheet.write_row(0, 1, INPUT_SIZES.iter().map(|item| (item.to_string() + " bajtů").to_owned()).collect::<Vec<String>>())?;
    for result in results{
        worksheet.write(i, 0, result.function_name)?;
        worksheet.write_row(i, 1, result.hash_counts)?;
        i +=1;
    }

    workbook.save("tabulka_vysledku.xlsx")?;
    println!("\nBenchmark dokončen.");

    Ok(())
}