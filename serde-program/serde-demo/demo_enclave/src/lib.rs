#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use serde::{Serialize, Deserialize};


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

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[no_mangle]
pub extern "C" fn serde_demo(x_coord: usize, y_coord: usize) -> sgx_status_t {
    let point = Point { x: x_coord, y: y_coord };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);



    sgx_status_t::SGX_SUCCESS
}
