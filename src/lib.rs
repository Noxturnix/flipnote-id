#[macro_use]
extern crate structure;

use hex::decode;

#[derive(Debug)]
pub struct FlipnoteId {
    pub id: u64,
    pub checksum: u16,
}

#[derive(Debug)]
pub enum FlipnoteDataError {
    InvalidSize,
}

fn check_data_length(data: &Vec<u8>) -> bool {
    return data.len() == 256;
}

pub fn extract_id_with_checksum(data: &Vec<u8>) -> Result<FlipnoteId, FlipnoteDataError> {
    if !check_data_length(data) {
        return Err(FlipnoteDataError::InvalidSize);
    }

    let flipnote_option_structure = structure!("<QH");
    let flipnote_id = flipnote_option_structure.unpack(&data[16..26]).unwrap();

    return Ok(FlipnoteId {
        id: flipnote_id.0,
        checksum: flipnote_id.1,
    });
}

pub fn compute_checksum(data: &Vec<u8>) -> Result<u16, FlipnoteDataError> {
    if !check_data_length(data) {
        return Err(FlipnoteDataError::InvalidSize);
    }

    let data_without_checksum = [&data[..24], &[0u8; 2], &data[26..]].concat();
    let mut checksum = 0u16;

    for (i, b) in data_without_checksum.iter().enumerate() {
        checksum += (b ^ i as u8) as u16;
    }

    return Ok(checksum);
}

pub fn decode_fsid(fsid: &String) -> Result<u64, ()> {
    if fsid.len() != 16 {
        return Err(());
    }

    let fsid_bytes = decode(fsid);

    if let Err(_) = fsid_bytes {
        return Err(());
    }

    let fsid_bytes = fsid_bytes.unwrap();

    match &fsid_bytes[0] >> 4 {
        0 | 1 | 5 | 9 => {}
        _ => return Err(()),
    }

    if &fsid_bytes[3] & 0b1111 != 0 {
        return Err(());
    }

    let fsid_structure = structure!("Q");
    return Ok(fsid_structure.unpack(fsid_bytes).unwrap().0);
}

pub fn set_fsid(data: &Vec<u8>, fsid: &String) -> Result<Vec<u8>, ()> {
    let fsid_num = decode_fsid(fsid);

    if let Err(_) = fsid_num {
        return Err(());
    }

    let fsid_num = fsid_num.unwrap();
    let fsid_structure = structure!("<Q");
    let new_data = [
        &data[..16],
        &fsid_structure.pack(fsid_num).unwrap(),
        &data[24..],
    ]
    .concat();

    let checksum = compute_checksum(&new_data).unwrap();
    let checksum_structure = structure!("<H");
    let new_data = [
        &new_data[..24],
        &checksum_structure.pack(checksum).unwrap(),
        &new_data[26..],
    ]
    .concat();

    return Ok(new_data);
}
