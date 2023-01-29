

use getset::{Getters, Setters};
use alloc::vec::Vec;


#[derive(Debug, Getters)]
pub struct Package { 
    #[getset(get = "pub")]
    payload_size: u32,

    #[getset(get = "pub")]
    payload: Vec<u8>, 

    // Data for iteration
    cur: usize,
    package_size_byte_encode: Vec<u8>,
 }
 impl Package {

    pub fn new(payload_size: u32) -> Self {
        Self {
            payload: Vec::with_capacity(payload_size as usize),
            payload_size: payload_size,
            cur: 0,
            package_size_byte_encode: Vec::new(),
        }
    }

    pub fn add_bytes_to_payload(&mut self, start_read_pos: u32, in_bytes: &Vec<u8> ) -> u32 {
        let missing_bytes_in_payload = self.payload_size - self.payload.len() as u32;
        let max_bytes_in = in_bytes.len() as u32 - start_read_pos;
        let number_read_bytes = if missing_bytes_in_payload < max_bytes_in { missing_bytes_in_payload} else {max_bytes_in};
        for n in 0..number_read_bytes {
            self.payload.push(in_bytes[(start_read_pos + n) as usize]);
        }
        number_read_bytes
    }

    pub fn is_payload_complete(&self) -> bool {
        self.payload.len() >= self.payload_size as usize
    }

    pub fn reset_iterator(&mut self) {
        self.cur = 0;
        self.package_size_byte_encode = payload_size_to_byte_stream(self.payload_size);

    }

    pub fn create_bytestram(&mut self) -> Vec<u8> {
        self.reset_iterator();
        let capacity = self.package_size_byte_encode.len() + self.payload.len();
        let mut v: Vec<u8> = Vec::with_capacity(capacity);
        let mut n = self.next();
        while n.is_some() {
            v.push(n.unwrap());
            n = self.next();
        }
        v
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;
        if self.payload.len() == other.payload.len() {
            for i in 0..self.payload.len() {
                if self.payload[i] != other.payload[i] {
                    eq = false;
                    break;
                }
            }
        } else {
            eq = false;
        }
        return eq;
    }
}
impl Eq for Package {}

impl Iterator for Package {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.payload_size == 0 {
            return None;
        } 
        let value: u8;
        if self.cur < self.package_size_byte_encode.len() {
            value = self.package_size_byte_encode[self.cur];
        } else {
            let payload_pos = self.cur - self.package_size_byte_encode.len();
            if payload_pos < self.payload.len() {
                value = self.payload[payload_pos];
            } else {
                return None;
            }
        }
        self.cur += 1;
        Some(value)
    }
}


 #[derive(Debug, Getters, Setters)]
 pub struct PackageInfo {
    #[getset(get = "pub", set = "pub")]
    start_pos: u32,
    #[getset(get = "pub", set = "pub")]
    package_size: u32,
    #[getset(get = "pub", set = "pub")]
    info_length_in_bytes: u32,
 }
 impl PackageInfo {
    pub fn new(start_pos: u32) -> Self {
        Self {
            start_pos,
            package_size: 0,
            info_length_in_bytes: 0,
        }
    }

    pub fn get_payload_size(&self) -> u32 {
        return self.package_size - self.info_length_in_bytes;
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ReadStreamResult {
    Success,
    TooFewBytes,
    PositionOutOfScope,
    CorruptStream,
    FailUnknownReason
}


pub fn payload_size_to_byte_stream(payload_size: u32) -> Vec<u8> {
    let mut vec = Vec::with_capacity(2);
    let mut bytes_for_package_size = 1;
    let mut max_package_size = (1<<(7 * bytes_for_package_size)) - 1;
    let mut package_size = payload_size;
    while package_size > max_package_size  {
        bytes_for_package_size += 1;
        max_package_size = (1<<(7 * bytes_for_package_size)) - 1;
    }
    if payload_size + bytes_for_package_size > max_package_size {
        bytes_for_package_size += 1;
    }
    package_size += bytes_for_package_size;

    for shift_factor in 0..bytes_for_package_size {
        let v = ((package_size >> (7 * shift_factor)) & 0x7F) as u8;
        vec.push(v);
    }
    let last_byte = vec.len() - 1;
    vec[last_byte] |= 0x80;
    vec
}

pub fn byte_stream_to_u32(stream: &Vec<u8>, package_stream_info: &mut PackageInfo) -> ReadStreamResult {
    let start_pos = *package_stream_info.start_pos();
    if stream.len() > start_pos as usize  {
        let mut pos = 0;
        let mut not_found_end_pos = true;
        let mut u32_value: u32 = 0;
        while start_pos + pos < (stream.len() as u32) && not_found_end_pos {
            let mut value: u32 = stream[(start_pos + pos) as usize] as u32;
            if (value & 0x80 ) != 0 {
                not_found_end_pos = false;
            }
            value = (value & 0x7F) << (pos * 7);
            u32_value = u32_value | value;
            pos += 1;
        }
        if pos > 5 {
            return ReadStreamResult::CorruptStream;
        }
        if not_found_end_pos {
            return ReadStreamResult::TooFewBytes;
        }
        package_stream_info.set_info_length_in_bytes(pos);
        package_stream_info.set_package_size(u32_value);
        return ReadStreamResult::Success;
    }
    ReadStreamResult::PositionOutOfScope
}


pub fn create_test_package(payload_size: u32) -> Package {
    let mut vec = Vec::with_capacity(payload_size as usize);

    for n in 0..payload_size {
        vec.push((n % 255) as u8);
    }

    let mut pac = Package::new(payload_size);
    pac.add_bytes_to_payload(0, &vec);

    return pac
}