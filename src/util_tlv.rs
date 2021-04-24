use std::io::{Write, Read};
use crate::XResult;

pub struct Tlv {
    pub r#type: u16,
    pub length: u32,
    pub value: Vec<u8>,
}

impl Tlv {
    pub fn new_empty(ty: u16) -> Self {
        Self {
            r#type: ty,
            length: 0,
            value: vec![],
        }
    }

    pub fn new(ty: u16, value: Vec<u8>) -> Self {
        assert!(value.len() < u32::MAX as usize, "Value too huge");
        Self {
            r#type: ty,
            length: value.len() as u32,
            value,
        }
    }

    pub fn compose(&self) -> Vec<u8> {
        let cap = self.value.len() + 4 + 2;
        let mut v = Vec::with_capacity(cap);
        v.extend_from_slice(&self.r#type.to_be_bytes());
        v.extend_from_slice(&self.length.to_be_bytes());
        v.extend_from_slice(self.value.as_slice());
        v
    }

    pub fn write<T>(&self, mut w: T) -> XResult<usize> where T: Write {
        let mut len = w.write(&self.r#type.to_be_bytes())?;
        len += w.write(&self.length.to_be_bytes())?;
        len += w.write(self.value.as_slice())?;
        Ok(len)
    }

    pub fn read<T>(mut r: T) -> XResult<Self> where T: Read {
        let mut r#type = [0_u8; 2];
        r.read_exact(&mut r#type)?;
        let mut length = [0_u8; 4];
        r.read_exact(&mut length)?;
        let len = u32::from_be_bytes(length);
        let mut value = vec![0_u8; len as usize];
        r.read_exact(&mut value)?;
        Ok(Self {
            r#type: u16::from_be_bytes(r#type),
            length: len,
            value,
        })
    }
}

#[test]
fn test_tlv() {
    {
        let tlv = Tlv::new_empty(0);
        assert_eq!([0, 0, 0, 0, 0, 0], tlv.compose().as_slice());
    }
    {
        let tlv = Tlv::new_empty(1);
        assert_eq!([0, 1, 0, 0, 0, 0], tlv.compose().as_slice());
    }
    {
        let tlv = Tlv::new_empty(256);
        assert_eq!([1, 0, 0, 0, 0, 0], tlv.compose().as_slice());
    }
    {
        let tlv = Tlv::new(1, vec![0]);
        assert_eq!([0, 1, 0, 0, 0, 1, 0], tlv.compose().as_slice());
    }
    {
        let tlv = Tlv::new(2, vec![1, 2]);
        assert_eq!([0, 2, 0, 0, 0, 2, 1, 2], tlv.compose().as_slice());
    }
    {
        let tlv = Tlv::new(2, vec![1, 2]);
        let bs = tlv.compose().clone();
        let tlv2 = Tlv::read(bs.as_slice()).unwrap();
        assert_eq!(bs, tlv2.compose());
        assert_eq!(2, tlv2.r#type);
        assert_eq!(2, tlv2.length);
        assert_eq!([1, 2], tlv2.value.as_slice());
    }
}

