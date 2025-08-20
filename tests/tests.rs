use ckbfs_types::{CKBFSData, CKBFSDataNative};
use molecule::prelude::Entity;
#[test]
fn test_ckbfs_hex() {
    let data = CKBFSDataNative {
        index: 1,
        checksum: 3839560431,
        content_type: String::from("plain/text"),
        filename: String::from("hello_ckbfs.txt"),
    };

    let raw_data: CKBFSData = data.into();

    println!("0x{}", hex::encode(raw_data.as_bytes().as_ref()));
}
