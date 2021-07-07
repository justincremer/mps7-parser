use nom;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct Data {
    header: Header,
    records: Vec<Record>,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub magic: u32,
    pub version: u8,
    pub record_count: u32,
}

#[derive(Debug, PartialEq)]
pub struct Record {
    pub type_id: RecordType,
    pub timestamp: u32,
    pub user_id: u64,
    pub amount: Option<f64>,
}

#[derive(Debug, PartialEq)]
pub enum RecordType {
    Debit(f64),
    Credit(f64),
    StartAutopay,
    EndAutopay,
}

impl Data {
    fn validate() -> bool {
        false
    }
}
