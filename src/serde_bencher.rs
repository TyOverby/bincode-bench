use super::SliceSerDeBench;
use serde::{Serialize, Deserialize};
use serde_bench;

#[derive(Debug)]
pub struct SerdeBench;

impl <T: Serialize + Deserialize> SliceSerDeBench<T> for SerdeBench {
    fn ser_vec(i: &T) -> Vec<u8> {
        serde_bench::serialize(i).unwrap()
    }

    fn de_slice(i: &[u8]) -> T {
        serde_bench::deserialize(i).unwrap()
    }
}
