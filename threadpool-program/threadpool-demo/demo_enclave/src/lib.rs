#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::panic;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

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
pub extern "C" fn threadpool_demo() -> sgx_status_t {
    println!("threadpool-demo");

    let n_workers = 4;
    let n_jobs = 8;

    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1).expect("channel will be there");
        });
    }
    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
    // Ocall to normal world for output
    println!("threadpool-demo");



    sgx_status_t::SGX_SUCCESS
}
