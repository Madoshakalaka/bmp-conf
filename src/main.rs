use std::{fs::File, io::{Seek, SeekFrom, Read, Write as _}};
use crc::*;


const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0x0000,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0xbb3d,
    residue: 0x0000,
};

fn main() {
    let mut file = File::open("CONFIG.BIN").unwrap();
    
    let new_file_name = "CONFIG.BIN.new";
    std::fs::remove_file(new_file_name).unwrap_or_default();

    let mut new_file = File::create_new(new_file_name).unwrap();
    // copy CONFIG.BIN to CONFIG.BIN.new
    file.seek(SeekFrom::Start(0)).unwrap();
    std::io::copy(&mut file, &mut new_file).unwrap();


    // I believe the slave simply ignores one of the interval settings.

    // peripheral
    let buf = [std::env::args().nth(1).unwrap().parse::<u8>().unwrap()];

    new_file.seek(SeekFrom::Start(3982)).unwrap();
    new_file.write(&buf).unwrap();
    new_file.seek(SeekFrom::Start(3984)).unwrap();
    new_file.write(&buf).unwrap();

    // central 
    let buf = [std::env::args().nth(2).unwrap_or("30".to_string()).parse::<u8>().unwrap()];
    new_file.seek(SeekFrom::Start(3988)).unwrap();
    new_file.write(&buf).unwrap();
    new_file.seek(SeekFrom::Start(3990)).unwrap();
    new_file.write(&buf).unwrap();

    let crc = Crc::<u16>::new(&CUSTOM_ALG);
    let mut buf = [0; 4092];
    new_file.seek(SeekFrom::Start(0)).unwrap();
    new_file.read(&mut buf).unwrap();

    let result = crc.checksum(&buf);

    let mut buf = [0; 2];
    buf[0] = (result & 0xff) as u8;
    buf[1] = (result >> 8) as u8;
    new_file.seek(SeekFrom::Start(4092)).unwrap();
    new_file.write(&buf).unwrap();


}
