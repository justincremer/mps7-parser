use nom::number::complete::le_f64;
use nom::IResult;
use std::convert::TryInto;
use std::mem::transmute;
use std::vec::Vec;

type Byte = u8;
type Rune = u32;

#[derive(Debug, PartialEq)]
pub struct Data {
    header: Header,
    records: Vec<Record>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub magic: Rune,        // 0..3
    pub version: Byte,      // 4
    pub record_count: Rune, // 5..8
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub type_id: RecordType, // 0
    pub timestamp: Rune,     // 1..4
    pub user_id: u64,        // 5..12
    pub amount: Option<f64>, // 13..20
}

fn parse_record(i: &[Byte]) -> IResult<&[Byte], Record, ()> {
    match get_transaction_amount(i) {
        Ok(a) => Record {
            type_id: i[0],
            timestamp: i[1..4],
            user_id: i[5..12],
            amount: a,
        },
        Err(s) => (),
    }
}

fn get_transaction_amount(i: &[Byte]) -> Result<Option<f64>, &str> {
    match i[0] {
        0x00 | 0x01 => {
            let res: Result<[Byte; 8], _> = i[13..20].try_into();
            match res {
                Ok(v) => Ok(Some(transmute::<[Byte; 8], f64>(v))),
                Err(_) => Err("Failed to parse record amount"),
            }
        }
        _ => Ok(None),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RecordType {
    Debit,
    Credit,
    StartAutopay,
    EndAutopay,
}
