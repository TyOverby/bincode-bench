use super::{SerDeBench, SliceSerDeBench};
use std::io::{Read, Write};
use bincode::{self, Infinite};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct BincodeBench;

impl <T: Serialize + Deserialize> SerDeBench<T> for BincodeBench {
    fn ser<W: Write>(i: &T, w: &mut W) {
        bincode::serialize_into(w, i, Infinite).unwrap()
    }

    fn de<R: Read>(r: &mut R) -> T {
        bincode::deserialize_from(r, Infinite).unwrap()
    }
}

impl <T: Serialize + Deserialize> SliceSerDeBench<T> for BincodeBench {
    fn ser_vec(i: &T) -> Vec<u8> {
        bincode::serialize(i, Infinite).unwrap()
    }

    fn de_slice(i: &[u8]) -> T {
        bincode::deserialize(i).unwrap()
    }
}
