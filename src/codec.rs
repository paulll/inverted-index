extern crate turbopfor_sys;

use turbopfor_sys::*;
use std::convert::TryInto;


/**
 * Block format:
 * u32 - n_items,
 * u8[] - turbopfor_data
 */

pub fn decode_block(file_data: &[u8]) -> Vec<u32> {
    if file_data.len() == 0 {
        return vec![];
    }

    if file_data.len() == 2*4 {
        let s = u32::from_le_bytes(file_data[0..4].try_into().unwrap());
        if s == 1 {
            return vec![u32::from_le_bytes(file_data[4..8].try_into().unwrap())];
        }
    }

    let amount = u32::from_le_bytes(file_data[0..4].try_into().unwrap()) ;
    let mut decoded_data = vec![0u32; (amount + 32).try_into().unwrap()];

    unsafe {
        let _ = p4nd1dec256v32(file_data.as_ptr().add(4) as *const ::std::os::raw::c_uchar, amount.try_into().unwrap(), decoded_data.as_mut_ptr() as *mut u32);
        decoded_data.resize(amount.try_into().unwrap(), 0);
    }

    return decoded_data;
}

pub fn encode_block(data: &[u32]) -> Vec<u8> {
    if data.len() == 0 {
        return vec![];
    }

    if data.len() == 1 {
        let mut result = vec![0u8; 8];
        let (s, d) = result.split_at_mut(4);
        s.copy_from_slice(&1u32.to_le_bytes());
        d.copy_from_slice(&data[0].to_le_bytes());
        return result;
    }

    let p4enc_bound = (data.len()+127)/128+(data.len()+32)*4;
    let mut encoded_data = vec![0u8; p4enc_bound + 4];
    let amount = data.len() as u32;
    let (ed_left, _) = encoded_data.split_at_mut(4);
    ed_left.copy_from_slice(&amount.to_le_bytes());

    unsafe {
        let size = p4nd1enc256v32(data.as_ptr() as *const u32, data.len(), encoded_data.as_mut_ptr().add(4) as *mut ::std::os::raw::c_uchar);
        encoded_data.resize(size+4, 0);
    }

    return encoded_data;
}