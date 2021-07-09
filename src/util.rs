use std::convert::TryInto;
use std::str;

pub fn bytes_to_u32(i: &[u8]) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(u32::from_be_bytes(i.try_into()?))
}

pub fn bytes_to_u64(i: &[u8]) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(u64::from_be_bytes(i.try_into()?))
}

pub fn bytes_to_str<'a>(i: &'a [u8]) -> Result<&'a str, Box<dyn std::error::Error>> {
    Ok(str::from_utf8(i)?)
}

pub fn bytes_to_f64(i: &[u8]) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(f64::from_be_bytes(i.try_into()?))
}
