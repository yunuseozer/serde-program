# my secret app

mysecretapp : the trusted portion mysecretapp_enclave will be using important-library [sgx_enclave], untrusted portion is just a driver for the secret portion.

Build with the commond "`cargo build`"

Then cd `target/debug/` or `target/release/`, run `./mysecretapp`