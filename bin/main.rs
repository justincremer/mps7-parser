extern crate mps7_parser;

use mps7_parser::parse::Data;
use std::time::Instant;

fn main() {
    let file_name = "test_data/txnlog.dat";
    let start_time = Instant::now();
    let data = Data::load(file_name);
    let total_time = Instant::now() - start_time;
    println!("{}", data);
    println!(
        "\ndeserialized {} records in {} nanoseconds",
        data.records.len(),
        total_time.as_nanos()
    );
}
