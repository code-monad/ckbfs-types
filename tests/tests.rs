use ckbfs_types::{BackLinkNative, CKBFSData, CKBFSDataNative};
use molecule::prelude::Entity;
#[test]
fn test_ckbfs_hex() {
    let data = CKBFSDataNative {
        indexes: vec![1],
        checksum: 3839560431,
        content_type: String::from("plain/text"),
        filename: String::from("hello_ckbfs.txt"),
        backlinks: vec![
            BackLinkNative {
                indexes: vec![0x1],
                checksum: 300548862,
                tx_hash: hex::decode(
                    "acea79e1cbbc2cf39bf4071fd0c73f5872255a9ec8532c270d47c1c26be88985",
                )
                .unwrap()
                .try_into()
                .unwrap(),
            },
            BackLinkNative {
                indexes: vec![0x1],
                checksum: 1014105452,
                tx_hash: hex::decode(
                    "2cf42e07c377f5cc2784ffb1c4e75ce51f7f291d91ff34d58c8f674de3b92060",
                )
                .unwrap()
                .try_into()
                .unwrap(),
            },
        ],
    };

    let raw_data: CKBFSData = data.into();

    println!("0x{}", hex::encode(raw_data.as_bytes().as_ref()));
}
