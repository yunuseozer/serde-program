#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(feature = "sgx_enclave")]
use serde_sgx as serde;

#[cfg(feature = "sgx_enclave")]
use serde_json_sgx as serde_json;

use lazy_static::lazy_static; 
use sgx_types::*;
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::SgxMutex;
use std::string::String;
use std::borrow::ToOwned;
use std::vec::Vec;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

use core::hash::Hasher;
use twox_hash::XxHash64;

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

    pub fn to_json(&self) -> String {
        // Serialize it to a JSON string.
        let json_string = serde_json::to_string(self).unwrap();
        println!("{}", json_string);
        json_string
    }
}

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

lazy_static! {
    static ref MY_IMPORTANT_OBJ: SgxMutex<MyImportantObj> = {
        let my_important_obj = MyImportantObj {
            my_u128_value: 0,
            my_string_value: String::from("")
        };
        SgxMutex::new(my_important_obj)
    };
}

#[no_mangle]
pub unsafe extern "C" fn my_important_obj_init(
    my_u128_value: *const u8,
    my_string_value: *const c_char
) -> sgx_status_t {
    if let Ok(mut my_important_obj) = MY_IMPORTANT_OBJ.lock() {
        let u128_value: [u8; 16] = core::slice::from_raw_parts(my_u128_value, 16).try_into().unwrap();
        my_important_obj.my_u128_value = u128::from_be_bytes(u128_value);
        my_important_obj.my_string_value = CStr::from_ptr(my_string_value).to_str().unwrap().to_owned();
    }
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn my_important_obj_hash(
    hash_result: *mut u8,
    hash_size: *mut usize
) -> sgx_status_t {
    let mut ret = [0_u8; 8];
    if let Ok(my_important_obj) = MY_IMPORTANT_OBJ.lock() {
        ret = my_important_obj.hash();
    }
    *hash_size = 8;
    let ret_buf_slice = core::slice::from_raw_parts_mut(hash_result, *hash_size);
    ret_buf_slice[..8].copy_from_slice(&ret);
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn my_important_obj_to_json(
    json_result: *mut u8,
    json_init_size: usize,
    json_size: *mut usize
) -> sgx_status_t {
    let mut ret: String = String::from("");
    if let Ok(my_important_obj) = MY_IMPORTANT_OBJ.lock() {
        ret = my_important_obj.to_json();
    }
    let ret_array = ret.as_bytes();
    let len = ret_array.len();
    let ret_buf_slice = core::slice::from_raw_parts_mut(json_result, json_init_size);
    ret_buf_slice[..len].copy_from_slice(ret_array);
    *json_size = len;
    sgx_status_t::SGX_SUCCESS
}
