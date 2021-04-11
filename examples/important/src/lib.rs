#![cfg_attr(any(feature = "sgx_enclave"), no_std)]

#[cfg(feature = "sgx_enclave")]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(feature = "std")]
use serde;
#[cfg(feature = "std")]
use serde_json;
#[cfg(feature = "std")]
use std::hash::Hasher;
#[cfg(feature = "std")]
use twox_hash::XxHash64;

#[cfg(feature = "sgx_enclave")]
use serde_sgx as serde;
#[cfg(feature = "sgx_enclave")]
use serde_json_sgx as serde_json;
#[cfg(feature = "sgx_enclave")]
use core::hash::Hasher;
#[cfg(feature = "sgx_enclave")]
use twox_hash_sgx::XxHash64;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

use std::string::String;
use std::vec::Vec;
use std::borrow::ToOwned;

#[cfg_attr(feature = "sgx_enclave", serde(crate = "serde_sgx"))]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MyImportantObj {
    #[serde(serialize_with = "serialize_u128")]
    #[serde(deserialize_with = "deserialize_u128")]
    pub my_u128_value: u128,
    #[serde(serialize_with = "serialize_string")]
    #[serde(deserialize_with = "deserialize_string")]
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
        let u128_vec: [u8; 16] = self.my_u128_value.to_le_bytes();
        let string_vec = self.my_string_value.as_bytes();
        let mut data_vec: Vec<u8> = Vec::new();
        for data in u128_vec.iter() {
            data_vec.push(*data);
        }
        for data in string_vec.iter() {
            data_vec.push(*data);
        }
        println!("MyImportantObj hash(), data_vec = {:?}", data_vec);
        hasher.write(data_vec.as_slice());
        hasher.finish().to_le_bytes()
    }

    pub fn to_json(&self) -> String {
        // Serialize it to a JSON string.
        let json_string = serde_json::to_string(self).unwrap();
        println!("MyImportantObj to_json(), json_string = {:?}", json_string);
        json_string
    }
}

// Implementation about serialze and deserialize
pub fn serialize_u128<S>(item: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let item_str = format!("{}", item);
    serializer.serialize_str(&item_str)
}

pub fn deserialize_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(parse_string_u128(&s))
}

pub fn parse_string_u128(u128_str: &str) -> u128 {
    if u128_str.starts_with("0x") {
        u128::from_str_radix(u128_str, 16).unwrap()
    } else {
        u128::from_str_radix(u128_str, 10).unwrap()
    }
}

pub fn serialize_string<S>(item: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&item)
}

pub fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_important_obj() {
        let expected_hash_value: [u8; 8] = [156, 168, 10, 178, 231, 61, 232, 29];
        let expected_json_string = "{\"my_u128_value\":\"0\",\"my_string_value\":\"abc\"}";
        let obj = MyImportantObj::new(0, "abc");
        let obj_hash = obj.hash();
        let obj_json_string = obj.to_json();
        println!("{:?}", obj_hash);
        println!("{:?}", obj_json_string);
        assert_eq!(expected_hash_value, obj_hash);
        assert_eq!(expected_json_string, obj_json_string);
    }
}
