#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::vec::Vec;

/// A simple function to calculate and print the fibonacci sequence
///
/// # Parameter
///
/// **len**
///
/// The length of fibonacci sequence
///
/// # Return value
///
/// The function always returns SGX_SUCCESS

#[no_mangle]
pub extern "C" fn fibonacci_seq(len: usize) -> sgx_status_t {
    if len >= 48 {
        println!("Unable to calculate Fibonacci sequence within {} length due to overflow", len);
        return sgx_status_t::SGX_SUCCESS;
    }

    let mut fibonacci_vec:Vec<u32> = Vec::new();

    match len {
        0 => {
        },
        1 => {
            fibonacci_vec.push(1);
        },
        2 => {
            fibonacci_vec.push(1);
            fibonacci_vec.push(1);
        },
        _ => {
            fibonacci_vec.push(1);
            fibonacci_vec.push(1);
            for i in 2..len as usize {
                fibonacci_vec.push(fibonacci_vec[i-1] + fibonacci_vec[i-2])
            }
        }
    }

    // Ocall to normal world for output
    println!("Fibonacci sequence length : {}", len);
    println!("Fibonacci sequence : {:?}", fibonacci_vec);

    sgx_status_t::SGX_SUCCESS
}
