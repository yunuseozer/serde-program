#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use lazy_static::lazy_static; 
use sgx_types::*;
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::SgxMutex;
use std::string::String;

use important::*;

lazy_static! {
    static ref MY_IMPORTANT_OBJ: SgxMutex<Option<MyImportantObj>> = {
        SgxMutex::new(None)
    };
}

#[no_mangle]
pub unsafe extern "C" fn my_important_obj_init(
    my_u128_value: *const u8,
    my_string_value: *const c_char
) -> sgx_status_t {
    if let Ok(mut obj) = MY_IMPORTANT_OBJ.lock() {
        let u128_value: [u8; 16] = core::slice::from_raw_parts(my_u128_value, 16).try_into().unwrap();
        let new_my_important_obj = MyImportantObj::new(
            u128::from_be_bytes(u128_value),
            CStr::from_ptr(my_string_value).to_str().unwrap()
        );
        *obj = Some(new_my_important_obj);
    }
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn my_important_obj_hash(
    hash_result: *mut u8,
    hash_size: *mut usize
) -> sgx_status_t {
    let mut ret = [0_u8; 8];
    if let Ok(obj) = MY_IMPORTANT_OBJ.lock() {
        match &*obj {
            Some(my_important_obj) => {
                ret = my_important_obj.hash();
            },
            None => {
                return sgx_status_t::SGX_ERROR_INVALID_STATE;
            },
        }
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
    if let Ok(obj) = MY_IMPORTANT_OBJ.lock() {
        match &*obj {
            Some(my_important_obj) => {
                ret = my_important_obj.to_json();
            },
            None => {
                return sgx_status_t::SGX_ERROR_INVALID_STATE;
            },
        }
    }
    let ret_array = ret.as_bytes();
    let len = ret_array.len();
    let ret_buf_slice = core::slice::from_raw_parts_mut(json_result, json_init_size);
    ret_buf_slice[..len].copy_from_slice(ret_array);
    *json_size = len;
    sgx_status_t::SGX_SUCCESS
}
