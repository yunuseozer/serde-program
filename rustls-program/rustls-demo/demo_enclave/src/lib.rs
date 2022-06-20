#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::vec::Vec;
use std::sync::Arc;

use std::net::TcpStream;

use rustls;
use webpki;
use webpki_roots;

use rustls::Session;
use io::Read;
use io::Write;
use io::stdout;
use std::io;



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
pub extern "C" fn rustls_demo() -> sgx_status_t {

    let mut config = rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let dns_name = webpki::DNSNameRef::try_from_ascii_str("google.com").unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::connect("google.com:443").unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);
    tls.write(concat!("GET / HTTP/1.1\r\n",
                      "Host: google.com\r\n",
                      "Connection: close\r\n",
                      "Accept-Encoding: identity\r\n",
                      "\r\n")
              .as_bytes())
        .unwrap();
    let ciphersuite = tls.sess.get_negotiated_ciphersuite().unwrap();
    writeln!(&mut std::io::stderr(), "Current ciphersuite: {:?}", ciphersuite.suite).unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();

    // Ocall to normal world for output
    println!("rustls---");

    sgx_status_t::SGX_SUCCESS
}
