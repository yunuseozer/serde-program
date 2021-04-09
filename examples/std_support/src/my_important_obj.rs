use std::hash::Hasher;

#[cfg(feature = "std")]
use serde;

// #[cfg(feature = "sgx_enclave")]
// use serde_sgx as serde;

use serde::{Deserialize, Serialize};
use twox_hash::XxHash64;

// #[cfg_attr(feature = "sgx_enclave", serde(crate = "serde_sgx"))]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MyImportantObj {
    pub my_u128_value: u128,
    pub my_string_value: String,
}

impl MyImportantObj {
    pub fn new(input_u128_value: u128, input_string_value: &str) -> Self {
        MyImportantObj {
            my_u128_value: input_u128_value,
            my_string_value: input_string_value.to_owned(),
        }
    }

    // performs a twox hash on the content
    // the context of MyImportantObj is defined as
    // [ my_u128_value : 16 bytes ][ my_string_value : my_string_value.len() bytes ]
    pub fn hash(&self) -> [u8; 8] {
        let mut hasher = XxHash64::with_seed(0);
        let u128_vec: [u8; 16] = self.my_u128_value.to_be_bytes();
        let string_vec = self.my_string_value.as_bytes();
        let mut data_vec: Vec<u8> = Vec::new();
        for data in u128_vec.iter() {
            data_vec.push(*data);
        }
        for data in string_vec.iter() {
            data_vec.push(*data);
        }
        println!("{:?}", data_vec);
        hasher.write(data_vec.as_slice());
        hasher.finish().to_be_bytes()
    }

    #[cfg(feature = "std")]
    pub fn to_json(&self) -> String {
        // Serialize it to a JSON string.
        let json_string = serde_json::to_string(self).unwrap();
        println!("{}", json_string);
        json_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_test() {
        let expected_hash_value: [u8; 8] = [29, 232, 61, 231, 178, 10, 168, 156];
        let expected_json_string = "{\"my_u128_value\":0,\"my_string_value\":\"abc\"}";
        let obj = MyImportantObj::new(0, "abc");
        let obj_hash = obj.hash();
        let obj_json_string = obj.to_json();
        println!("{:?}", obj_hash);
        println!("{:?}", obj_json_string);
        assert_eq!(expected_hash_value, obj_hash);
        assert_eq!(expected_json_string, obj_json_string);
    }
}