use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::types::Xid;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>);
}

pub fn serialize_u8_vec(res: &mut Vec<u8>, data: &Vec<u8>) {
    let content_size = data.len();
    serialize_usize(res, content_size);
    for d in data {
        res.write_u8(*d);
    }
}

pub fn deserialize_u8_vec(rdr: &mut Cursor<Vec<u8>>) -> Vec<u8> {
    let mut res = Vec::new();
    let size = deserialize_usize(rdr);
    for _ in 0..size {
        let b = rdr.read_u8().unwrap();
        res.push(b);
    }
    res
}

pub fn serialize_usize(res: &mut Vec<u8>, size: usize) {
    if cfg!(target_pointer_width = "64") {
        let size = size as u64;
        res.write_u64::<BigEndian>(size).unwrap();
    } else if cfg!(target_pointer_width = "32") {
        let size = size as u32;
        res.write_u32::<BigEndian>(size).unwrap();
    } else {
        let size = size as u16;
        res.write_u16::<BigEndian>(size).unwrap();
    }
}

pub fn deserialize_usize(rdr: &mut Cursor<Vec<u8>>) -> usize {
    if cfg!(target_pointer_width = "64") {
        let size = rdr.read_u64::<BigEndian>().unwrap();
        let size = size as usize;
        size
    } else if cfg!(target_pointer_width = "32") {
        let size = rdr.read_u32::<BigEndian>().unwrap();
        let size = size as usize;
        size
    } else {
        let size = rdr.read_u16::<BigEndian>().unwrap();
        let size = size as usize;
        size
    }
}

pub fn serialize_xid(res: &mut Vec<u8>, xid: &Xid) {
    // TODO
}

pub fn deserialize_xid(rdr: &mut Cursor<Vec<u8>>) -> Xid {
    // TODO
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn serde_u8_vec() {
        let v: Vec<u8> = vec!(1, 2, 3);
        // Serialize
        let mut res = Vec::new();
        serialize_u8_vec(&mut res, &v);
        // Deserialize
        let v_out = deserialize_u8_vec(&mut Cursor::new(res));
        assert_eq!(v_out, v);
    }
}
