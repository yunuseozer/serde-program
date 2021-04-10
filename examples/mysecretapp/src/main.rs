extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::ffi::CString;

static ENCLAVE_FILE: &'static str = "mysecretapp_enclave.signed.so";

include!(concat!(env!("OUT_DIR"), "/ecall.rs"));

fn init_enclave() -> SgxResult<SgxEnclave> {
    let debug = 1;
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t {
            flags: 0,
            xfrm: 0
        },
        misc_select:0
    };

    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let input_u128_value: u128 = 123;
    let input_string_value = CString::new("abc");

    let input_u128_value_enclave_param:[u8; 16] = input_u128_value.to_be_bytes();
    let input_string_value_enclave_param = input_string_value.unwrap();

    let mut return_value = sgx_status_t::SGX_SUCCESS;
    let result = unsafe {
        my_important_obj_init(enclave.geteid(),
                              &mut return_value,
                              input_u128_value_enclave_param.as_ptr(),
                              input_string_value_enclave_param.as_ptr())
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    let mut hash_result: Vec<u8> = vec![0_u8; 8];
    let mut hash_size: usize = 0;
    let result = unsafe {
        my_important_obj_hash(enclave.geteid(),
                              &mut return_value,
                              hash_result.as_mut_ptr(),
                              &mut hash_size)
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    if hash_size != 8 || hash_size != hash_result.len() {
        println!("[-] Executed hash function fail! The hash_result is {:?}, The hash_size is {:?}", hash_result, hash_size);
    }
    println!("[+] hash() function result: {:?}", hash_result);

    let mut json_result = [0_u8; 1024];
    let mut json_size: usize = 0;
    let result = unsafe {
        my_important_obj_to_json(enclave.geteid(),
                                 &mut return_value,
                                 json_result.as_mut_ptr(),
                                 json_result.len(),
                                 &mut json_size)
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    let mut json_real_size = 0;
    let mut json_bytes: Vec<u8> = Vec::new();
    for iter in json_result.iter() {
        if (*iter == 0) {
            break;
        }
        json_real_size = json_real_size + 1;
        json_bytes.push(*iter);
    }
    if json_size != json_real_size {
        println!("[-] Executed to_json function fail! The raw json_result is {:?}, The json_size is {:?}", json_result, json_size);
    }
    println!("[+] to_json() function result: {:?}", String::from_utf8_lossy(json_bytes.as_slice()));

    println!("[+] Enclave Program Completed...");
    enclave.destroy();
}
