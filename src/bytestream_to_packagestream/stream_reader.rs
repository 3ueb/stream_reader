use alloc::vec::Vec;
use getset::{Getters};
use crate::bytestream_to_packagestream::package::PackageInfo;
use crate::bytestream_to_packagestream::package::ReadStreamResult;
use crate::bytestream_to_packagestream::package::Package;
use crate::bytestream_to_packagestream::package;



pub struct Stream {
    temp_stream_storage: Vec<u8>,
    pos_start_reading: u32,
    pos_end_reading: u32,
    incomplet_package: Option<Package>,
}

impl Stream {
    pub fn new() -> Stream {
        Self {
            temp_stream_storage: Vec::new(),
            pos_start_reading: 0,
            pos_end_reading: 0,
            incomplet_package: None,
        }
    }    

    pub fn add_new_bytes_to_stream(&mut self, bytes: Vec<u8>) -> AddNewByteResult {
        if self.pos_start_reading == self.temp_stream_storage.len() as u32 {
            self.temp_stream_storage = bytes;
            self.pos_start_reading = 0;
            self.pos_end_reading = self.temp_stream_storage.len() as u32;
        } else {
            self.temp_stream_storage.extend(bytes.iter());
        }
        let mut packages: Vec<Package> = Vec::new();
        let mut left_bytes = 0;
        let mut read_more_package: bool = true;
        let mut read_stream_result= ReadStreamResult::FailUnknownReason; 

        
        let optional_inclompet_package = self.incomplet_package.take();
        if optional_inclompet_package.is_some() {
            let mut inclompet_package: Package = optional_inclompet_package.unwrap();
            let read_bytes = inclompet_package.add_bytes_to_payload(self.pos_start_reading, &self.temp_stream_storage);
            self.pos_start_reading += read_bytes;
            if inclompet_package.is_payload_complete() {
                packages.push(inclompet_package);
                left_bytes = self.temp_stream_storage.len() as u32 - self.pos_start_reading;   
            } else {
                left_bytes = 0;
                self.incomplet_package = Some(inclompet_package);
            }
        }

        while read_more_package {
            let mut package_stream_info: PackageInfo = PackageInfo::new(self.pos_start_reading);
            read_stream_result = package::byte_stream_to_u32(&self.temp_stream_storage, &mut package_stream_info);
            if  ReadStreamResult::Success == read_stream_result {
                let mut package = Package::new(package_stream_info.get_payload_size());
                self.pos_start_reading += *package_stream_info.info_length_in_bytes();
                let read_bytes = package.add_bytes_to_payload(self.pos_start_reading, &self.temp_stream_storage);
                self.pos_start_reading += read_bytes;
                if package.is_payload_complete() {
                    packages.push(package);
                    left_bytes = self.temp_stream_storage.len() as u32 - self.pos_start_reading;   
                } else {
                    left_bytes = 0;
                    self.incomplet_package = Some(package);

                }
                if left_bytes == 0 {
                    read_more_package = false;
                }
            } else {
                left_bytes = self.temp_stream_storage.len() as u32 - self.pos_start_reading;   
                read_more_package = false;
            }


        }
        
        AddNewByteResult { packages: packages, left_bytes: left_bytes, read_stream_result: read_stream_result }
    }
}

#[derive(Debug, Getters)]
pub struct AddNewByteResult {
    #[getset(get = "pub")]
    packages: Vec<Package>,

    #[getset(get = "pub")]
    left_bytes: u32,

    #[getset(get = "pub")]
    read_stream_result: ReadStreamResult,
}