#![no_std]
extern crate alloc;
pub use crate::generated::ckbfs::{Bytes, CKBFSData};
use alloc::{string::String, vec::Vec};

use generated::ckbfs::{BackLink, BackLinkVec, Byte32, Uint32, Uint32Opt};
use molecule::prelude::{Builder, Entity};

pub mod generated;

impl Into<Bytes> for &[u8] {
    fn into(self) -> Bytes {
        let len = self.len();
        let mut vec: Vec<u8> = Vec::with_capacity(4 + len);
        vec.extend_from_slice(&(len as u32).to_le_bytes()[..]);
        vec.extend_from_slice(self);
        Bytes::new_unchecked(Bytes::from_slice(vec.as_slice()).unwrap().as_bytes())
    }
}

#[derive(Debug, Clone)]
pub struct BackLinkNative {
    pub index: u32,
    pub checksum: u32,
    pub tx_hash: [u8; 32],
}

impl Into<BackLinkNative> for BackLink {
    fn into(self) -> BackLinkNative {
        let index = u32::from_le_bytes(self.index().as_slice().try_into().unwrap());
        let checksum = u32::from_le_bytes(self.checksum().as_slice().try_into().unwrap());
        let tx_hash = self.tx_hash().as_slice().try_into().unwrap();
        BackLinkNative {
            index,
            checksum,
            tx_hash,
        }
    }
}

impl Into<BackLink> for BackLinkNative {
    fn into(self) -> BackLink {
        BackLink::new_builder()
            .index(Uint32::new_unchecked(molecule::bytes::Bytes::from(
                self.index.to_le_bytes().to_vec(),
            )))
            .checksum(Uint32::new_unchecked(molecule::bytes::Bytes::from(
                self.checksum.to_le_bytes().to_vec(),
            )))
            .tx_hash(Byte32::new_unchecked(molecule::bytes::Bytes::from(
                self.tx_hash.to_vec(),
            )))
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct CKBFSDataNative {
    pub index: Option<u32>,
    pub checksum: u32,
    pub content_type: String,
    pub filename: String,
    pub backlinks: Vec<BackLinkNative>,
}

impl From<CKBFSDataNative> for CKBFSData {
    fn from(data: CKBFSDataNative) -> Self {
        let content_type = data.content_type.as_bytes().into();
        let filename = data.filename.as_bytes().into();
        let index = data
            .index
            .map(|x| Uint32::new_unchecked(molecule::bytes::Bytes::from(x.to_le_bytes().to_vec())));
        let backlinks = data
            .backlinks
            .into_iter()
            .map(|backlink| backlink.into())
            .collect::<Vec<BackLink>>();
        CKBFSData::new_builder()
            .index(Uint32Opt::new_builder().set(index).build())
            .checksum(Uint32::new_unchecked(molecule::bytes::Bytes::from(
                data.checksum.to_le_bytes().to_vec(),
            )))
            .filename(filename)
            .content_type(content_type)
            .backlinks(BackLinkVec::new_builder().extend(backlinks).build())
            .build()
    }
}

impl Into<CKBFSDataNative> for CKBFSData {
    fn into(self) -> CKBFSDataNative {
        let content_type = String::from_utf8(self.content_type().as_slice().to_vec())
            .expect("Failed to extract content-type");
        let filename = String::from_utf8(self.filename().as_slice().to_vec())
            .expect("Failed to extract filname");
        let index = match self.index().is_some() {
            true => Some(u32::from_le_bytes(
                self.index()
                    .to_opt()
                    .unwrap()
                    .as_slice()
                    .try_into()
                    .unwrap(),
            )),
            false => None,
        };
        let checksum = u32::from_le_bytes(self.checksum().as_slice().try_into().unwrap());
        let backlinks = self
            .backlinks()
            .into_iter()
            .map(|raw_backlink| raw_backlink.into())
            .collect::<Vec<BackLinkNative>>();
        CKBFSDataNative {
            index,
            checksum,
            content_type,
            filename,
            backlinks,
        }
    }
}
