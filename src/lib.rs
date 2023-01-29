#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

mod bytestream_to_packagestream;

#[no_mangle]
pub extern "C" fn create_stream() {
    let mut stream = bytestream_to_packagestream::stream_reader::Stream::new();
}


#[cfg(test)]
mod tests {
    use crate::bytestream_to_packagestream::package::{ReadStreamResult, PackageInfo, Package};

    use super::*;

    #[test]
    fn internal() {
        let package = bytestream_to_packagestream::package::create_test_package(4);
        assert_eq!(*package.payload_size(), 4);
        assert_eq!(package.payload().len(), 4); 
    }

    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    #[test]
    fn byte_stream_package_size() {
        let v = bytestream_to_packagestream::package::payload_size_to_byte_stream(1);
        assert_eq!(v[0], 0x82);

        let v = bytestream_to_packagestream::package::payload_size_to_byte_stream(126);
        assert_eq!(v[0], 0xFF);

        let v = bytestream_to_packagestream::package::payload_size_to_byte_stream(127);
        assert_eq!(v[0], 0x01);
        assert_eq!(v[1], 0x81);

        let v = bytestream_to_packagestream::package::payload_size_to_byte_stream(16381);
        assert_eq!(v[0], 0x7F);
        assert_eq!(v[1], 0xFF);

        let v = bytestream_to_packagestream::package::payload_size_to_byte_stream(16382);
        assert_eq!(v[0], 0x01);
        assert_eq!(v[1], 0x00);
        assert_eq!(v[2], 0x81);

        let text_input: Vec<u8> = vec![0x82];
        let start_pos = 2;
        let mut package_stream_info: PackageInfo = PackageInfo::new(start_pos);
        let read_stream_result = bytestream_to_packagestream::package::byte_stream_to_u32(&text_input, &mut package_stream_info);
        assert_eq!(read_stream_result, ReadStreamResult::PositionOutOfScope);

        let text_input: Vec<u8> = vec![0x82];
        let start_pos = 0;
        let mut package_stream_info: PackageInfo= PackageInfo::new(start_pos);
        let read_stream_result = bytestream_to_packagestream::package::byte_stream_to_u32(&text_input, &mut package_stream_info);
        assert_eq!(read_stream_result, ReadStreamResult::Success);
        assert_eq!(*package_stream_info.package_size(), 2);
        assert_eq!(*package_stream_info.info_length_in_bytes(), 1);

        let text_input: Vec<u8> = vec![0xFF];
        let start_pos = 0;
        let mut package_stream_info: PackageInfo= PackageInfo::new(start_pos);
        let read_stream_result = bytestream_to_packagestream::package::byte_stream_to_u32(&text_input, &mut package_stream_info);
        assert_eq!(read_stream_result, ReadStreamResult::Success);
        assert_eq!(*package_stream_info.package_size(), 127);
        assert_eq!(*package_stream_info.info_length_in_bytes(), 1);

    }

    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    #[test]
    fn byte_stream_package_size_long_run_test() {

        
        for test_value in 1..18000000 {
            let v = bytestream_to_packagestream::package::payload_size_to_byte_stream(test_value);

            let start_pos = 0;
            let mut value_info: PackageInfo= PackageInfo::new(start_pos);
            let read_stream_result = bytestream_to_packagestream::package::byte_stream_to_u32(&v, &mut value_info);
            assert_eq!(read_stream_result, ReadStreamResult::Success);
            assert_eq!(*value_info.package_size() - *value_info.info_length_in_bytes(), test_value);

        }
    }

    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    #[test]
    fn package_test() {
        let payload_size = 1;
        let mut vec = Vec::with_capacity(payload_size as usize);

        for n in 0..payload_size + 5 {
            vec.push((n % 255) as u8);
        }
    
        let mut pac = bytestream_to_packagestream::package::Package::new(payload_size);
        pac.add_bytes_to_payload(0, &vec);
        assert_eq!(true, pac.is_payload_complete());


    }


    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    #[test]
    fn stream_test() {

        let mut stream = bytestream_to_packagestream::stream_reader::Stream::new();

        let mut package = bytestream_to_packagestream::package::create_test_package(1);
        let test_bytes = package.create_bytestram();
        let mut add_new_bytes_result = stream.add_new_bytes_to_stream(test_bytes);
        assert_eq!(add_new_bytes_result.packages().len(), 1);


        
        let mut package = bytestream_to_packagestream::package::create_test_package(1);
        let test_bytes_all = package.create_bytestram();
        for b in test_bytes_all {
            let test_byte_part = vec![b];
            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
        }       
        assert_eq!(add_new_bytes_result.packages().len(), 1);


        let mut package = bytestream_to_packagestream::package::create_test_package(5);
        let test_bytes_all = package.create_bytestram();
        for b in test_bytes_all {
            let test_byte_part = vec![b];
            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
        }       
        assert_eq!(add_new_bytes_result.packages().len(), 1);
        assert_eq!(add_new_bytes_result.packages()[0].payload().len(), 5);


        let mut package1 = bytestream_to_packagestream::package::create_test_package(5);
        let mut package2 = bytestream_to_packagestream::package::create_test_package(8);

        let mut test_bytes_all = package1.create_bytestram();
        let mut test_bytes_all1 = package2.create_bytestram();
        let template:  Vec<Package> = vec![package1, package2];
        test_bytes_all.append(&mut test_bytes_all1);
        let mut template_index = 0;
        for b in test_bytes_all {
            let test_byte_part = vec![b];
            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
            for i in 0..add_new_bytes_result.packages().len() {
                assert_eq!(template[template_index], add_new_bytes_result.packages()[i]);
                template_index += 1;
            }
        }       


        let mut package1 = bytestream_to_packagestream::package::create_test_package(5);
        let mut package2 = bytestream_to_packagestream::package::create_test_package(255);

        let mut test_bytes_all = package1.create_bytestram();
        let mut test_bytes_all1 = package2.create_bytestram();
        let template:  Vec<Package> = vec![package1, package2];
        test_bytes_all.append(&mut test_bytes_all1);
        let mut template_index = 0;
        for b in test_bytes_all {
            let test_byte_part = vec![b];
            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
            for i in 0..add_new_bytes_result.packages().len() {
                assert_eq!(template[template_index], add_new_bytes_result.packages()[i]);
                template_index += 1;
            }
        }  

        let mut package1 = bytestream_to_packagestream::package::create_test_package(1000);
        let mut package2 = bytestream_to_packagestream::package::create_test_package(2);

        let mut test_bytes_all = package1.create_bytestram();
        let mut test_bytes_all1 = package2.create_bytestram();
        let template:  Vec<Package> = vec![package1, package2];
        test_bytes_all.append(&mut test_bytes_all1);
        let mut template_index = 0;
        for b in test_bytes_all {
            let test_byte_part = vec![b];
            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
            for i in 0..add_new_bytes_result.packages().len() {
                assert_eq!(template[template_index], add_new_bytes_result.packages()[i]);
                template_index += 1;
            }
        }  

        let mut package1 = bytestream_to_packagestream::package::create_test_package(1000);
        let mut package2 = bytestream_to_packagestream::package::create_test_package(2);

        let mut test_bytes_all = package1.create_bytestram();
        let mut test_bytes_all1 = package2.create_bytestram();
        let template:  Vec<Package> = vec![package1, package2];
        test_bytes_all.append(&mut test_bytes_all1);
        let mut template_index = 0;
        let max_test_vec_length = 100;
        let rounds = (test_bytes_all.len() / max_test_vec_length) + if((test_bytes_all.len() % max_test_vec_length != 0)) {1} else {0};
        for round in 0..rounds {
            let mut test_byte_part = Vec::new();
            let mut max_bytes = max_test_vec_length;
            if ((round + 1) * max_test_vec_length) >= test_bytes_all.len() {
                max_bytes = test_bytes_all.len() % max_test_vec_length;
            }
            let mut start = round * max_test_vec_length;
            for i in 0.. max_bytes {
                test_byte_part.push(test_bytes_all[start + i]);
            }

            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
            for i in 0..add_new_bytes_result.packages().len() {
                assert_eq!(template[template_index], add_new_bytes_result.packages()[i]);
                template_index += 1;
            }
        }  


        let mut package1 = bytestream_to_packagestream::package::create_test_package(2);
        let mut package2 = bytestream_to_packagestream::package::create_test_package(1000);

        let mut test_bytes_all = package1.create_bytestram();
        let mut test_bytes_all1 = package2.create_bytestram();
        let template:  Vec<Package> = vec![package1, package2];
        test_bytes_all.append(&mut test_bytes_all1);
        let mut template_index = 0;
        let max_test_vec_length = 100;
        let rounds = (test_bytes_all.len() / max_test_vec_length) + if((test_bytes_all.len() % max_test_vec_length != 0)) {1} else {0};
        for round in 0..rounds {
            let mut test_byte_part = Vec::new();
            let mut max_bytes = max_test_vec_length;
            if ((round + 1) * max_test_vec_length) >= test_bytes_all.len() {
                max_bytes = test_bytes_all.len() % max_test_vec_length;
            }
            let mut start = round * max_test_vec_length;
            for i in 0.. max_bytes {
                test_byte_part.push(test_bytes_all[start + i]);
            }

            add_new_bytes_result = stream.add_new_bytes_to_stream(test_byte_part);
            for i in 0..add_new_bytes_result.packages().len() {
                assert_eq!(template[template_index], add_new_bytes_result.packages()[i]);
                template_index += 1;
            }
        }

    }

}
