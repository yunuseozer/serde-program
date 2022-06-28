extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::io;

static ENCLAVE_FILE: &'static str = "demo_enclave.signed.so";

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


    let mut return_value = sgx_status_t::SGX_SUCCESS;

    let result = unsafe {
        threadpool_demo(enclave.geteid(),
                      &mut return_value)
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    println!("[+] fibonacci sequence enclave program completed...");
    enclave.destroy();
}
