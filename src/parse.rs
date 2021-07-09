use crate::util::{bytes_to_f64, bytes_to_str, bytes_to_u32, bytes_to_u64};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct Data {
    pub header: Header,
    pub records: Vec<Record>,
}

impl Data {
    pub fn load(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        match reader.read_to_end(&mut buffer) {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };

        let header = Header::new(buffer.take(9).into_inner());
        buffer = buffer.drain(9..).collect();

        let mut records: Vec<Record> = Vec::new();
        while !buffer.is_empty() {
            let res = RecordResult::new(&mut buffer);
            records.push(res.item);
            buffer = buffer.drain(res.bytes..).collect();
        }

        Data { header, records }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = format!(
            "{:?}\n{}",
            self.header,
            self.records
                .iter()
                .fold(String::new(), |acc, i| acc + format!("{:?}\n", i).as_str())
        );
        write!(f, "{}", res)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub magic: String,     // 0..3
    pub version: u8,       // 4
    pub record_count: u32, // 5..8
}

impl Header {
    pub fn new(i: &[u8]) -> Self {
        Header {
            magic: match bytes_to_str(&i[0..4]) {
                Ok(r) => r.to_string(),
                Err(s) => panic!("{}", s),
            },
            version: i[4],
            record_count: match bytes_to_u32(&i[5..9]) {
                Ok(r) => r,
                Err(s) => panic!("{}", s),
            },
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Record {
    pub type_id: RecordType, // 0
    pub timestamp: u32,      // 1..4
    pub user_id: u64,        // 5..12
    pub amount: Option<f64>, // pub amount: Option<f64>, // 13..20
}

impl Record {
    pub fn new(i: &[u8]) -> Self {
        let type_id = RecordType::from(i[0]);
        let amount = match type_id {
            RecordType::Debit | RecordType::Credit => match bytes_to_f64(&i[13..21]) {
                Ok(v) => Some(v),
                Err(e) => panic!("{}", e),
            },
            _ => None,
        };

        Record {
            type_id,
            timestamp: match bytes_to_u32(&i[1..5]) {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            },
            user_id: match bytes_to_u64(&i[5..13]) {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            },
            amount,
        }
    }
}

pub struct RecordResult {
    item: Record,
    bytes: usize,
}

impl RecordResult {
    fn new(i: &[u8]) -> Self {
        let item = Record::new(i);
        let bytes = match item.amount.is_some() {
            true => 21,
            false => 13,
        };

        RecordResult { item, bytes }
    }
}

#[derive(Debug, PartialEq)]
pub enum RecordType {
    Debit,
    Credit,
    StartAutopay,
    EndAutopay,
}

impl From<u8> for RecordType {
    fn from(i: u8) -> Self {
        match i {
            0x00 => RecordType::Debit,
            0x01 => RecordType::Credit,
            0x02 => RecordType::StartAutopay,
            0x03 => RecordType::EndAutopay,
            _ => panic!("record type invalid (malformed byte stream)"),
        }
    }
}
