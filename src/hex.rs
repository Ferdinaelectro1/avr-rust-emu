//Line format
//:LLAAAATT<data>CC
use std::{vec::Vec};
use crate::Cpu;

pub struct HexRecord {
    len: usize,
    address: u16,
    record_type: u8,
    data: Vec<u8>,
}

pub fn load_hex(hex: &str, cpu: &mut Cpu) -> Result<usize, String> {
    let records = parse_hex(hex)?;

    let mut bytes = vec![0xFF; 32 * 1024];
    let mut total_bytes: usize = 0;
    for record in records {
        match record.record_type {
            0x00 => { 
                let start_addr = record.address as usize;
                let end_addr = start_addr + record.len;
                if end_addr > bytes.len() {
                    return Err(String::from("Record exceeds flash size"));
                }
                bytes[start_addr..end_addr].copy_from_slice(&record.data);
                total_bytes += record.len;
            }
            0x01 => break,   // end of file
            _ => {}          // other type: ignore
        }
    }
    let words: Vec<u16> = bytes
    .chunks_exact(2)
    .map(|c| u16::from_le_bytes([c[0], c[1]]))
    .collect();
    cpu.load_program(&words);
    Ok(total_bytes)
}

pub fn parse_hex(hex: &str) -> Result<Vec<HexRecord>, String>{
    if hex.is_empty() {
        return Err(String::from("Hex is empty"));
    }
    let mut hexrecords = Vec::new();
    for line in hex.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        hexrecords.push(parse_line(line)?);
    }
    Ok(hexrecords)
}

fn parse_line(line: &str) -> Result<HexRecord,String> {
    if line.is_empty() {
        return Err(String::from("Line is empty"));
    }
    let mut line_bytes = Vec::new();   // créé à chaque tour de la boucle des lignes
    let content = match line.strip_prefix(':') {
        Some(c) => c,
        None => return Err(String::from("Line format is incorrect")),
    };
    if !content.is_ascii() {
        return Err(String::from("Non-ASCII character"));
    }
    if content.len() % 2 != 0 {
        return Err(String::from("Odd number of hex digits"));
    }
    for i in (0..content.len()).step_by(2) {
        let pair = &content[i..i+2];
        let byte = match u8::from_str_radix(pair, 16) {
            Ok(b) => b,
            Err(_) => return Err(String::from("Invalid hex digit"))
        }; 
        line_bytes.push(byte);
    }
    if line_bytes.len() < 5 || line_bytes.len() != line_bytes[0] as usize + 5 {
        return Err(String::from("Wrong line length"));
    
    }
    let mut sum : u8 = 0;
    for b in &line_bytes {
        sum = sum.wrapping_add(*b);
    }
    if sum != 0 {
        return Err(String::from("Bad checksum"));
    }
    let len = line_bytes[0] as usize;    // number of data octet
    let address = u16::from_be_bytes([line_bytes[1], line_bytes[2]]);
    let record_type = line_bytes[3];        // recording type
    let data = line_bytes[4..4 + len].to_vec();
    Ok(HexRecord { len, address, record_type, data })
}