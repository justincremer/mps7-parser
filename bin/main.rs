extern crate mps7_parser;

use mps7_parser::parse::{Data, RecordType};
use std::time::Instant;

fn main() {
    let file_name = "test_data/txnlog.dat";
    let start_time = Instant::now();
    let data = Data::load(file_name);
    let total_time = Instant::now() - start_time;
    println!("{}", data);
    println!(
        "deserialized {} records in {} nanoseconds",
        data.records.len(),
        total_time.as_nanos()
    );
    println!("{}", display_solution(data));
}

fn display_solution(data: Data) -> String {
    let total_credit = data.fold_by_record_type(RecordType::Credit);
    let total_debit = data.fold_by_record_type(RecordType::Debit);
    let total_started = data.fold_by_record_type(RecordType::StartAutopay) as u32;
    let total_ended = data.fold_by_record_type(RecordType::EndAutopay) as u32;
    let custom_user_balance = data
        .records
        .iter()
        .find(|r| r.user_id == 2456938384156277127)
        .unwrap()
        .amount
        .unwrap();

    format!(
        "
total credit amount={:.2}
total debit amount={:.2}
autopays started={}
autopays ended={}
balance for user 2456938384156277127={:.2}",
        total_credit, total_debit, total_started, total_ended, custom_user_balance
    )
}
