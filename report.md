**Intel SGX**



- Set of security-related instructions built into the CPU that provide a hardware-based Trusted Execution Environment (TEE)
- Allows a user- or kernel-level application to define (inside its address space) protected memory regions, called ***enclaves***
- Enclave memory cannot be read or written by processes outside the enclave itself regardless of their privilege levels and CPU mode; the only way to call an enclave function is through a new instruction that performs several protection checks
- The CPU protects the enclave by encrypting its memory with an encryption key stored inside the CPU that changes randomly every power cycle. The enclave is decrypted on the fly only within the CPU, and only for code and data within the enclave itself. The CPU limits memory access by enforcing checks on the TLB access and memory address translation, and automatically encrypting data when evicting pages to untrusted memory regions
- The contents of the enclaves and the associated data structures are stored inside the Enclave Page Cache (EPC). There is a hard limit on the EPC size, with typical values being 64 and 128 MB. Depending on the size of each enclave, we can expect between 5 and 20 enclaves to reside simultaneously in memory

- An SGX-enabled application has both a *trusted component* (the enclave) and an *untrusted component* (the rest of the application and its modules). The trusted component should be as small as possible in order to save protected memory and limit the attack surface. Applications should also minimize trusted-untrusted component interaction

 Intel SGX application execution flow (image taken from https://software.intel.com/content/www/us/en/develop/articles/intel-software-guard-extensions-tutorial-part-1-foundation.html)

![intel-software-guard-extensions-tutorial-intel-sgx-foundation-fig03-658687](/home/oana/Desktop/SGX/intel-software-guard-extensions-tutorial-intel-sgx-foundation-fig03-658687.png)

- The enclave code is enabled using special instructions and loaded as a dynamic library
- **Attestation** is the process of demonstrating that a specific enclave was established on a platform
  - Local attestation - two enclaves on the same platform authenticate each other
  - Remote attestation - an enclave gains the trust of a remote provider
- **Sealing data** is the process of encrypting data to be written to untrusted memory. The data can be read back in the enclave at a later date and unsealed (decrypted). The encryption keys are derived internally on demand and are not exposed to the enclave
- Applications running inside of SGX must be written to be side channel resistant as SGX does not protect against side channel measurement or observation



**Intel SGX SDK**



- The Intel SGX SDK is a collection of APIs, libraries, documentation, sample source code, and tools that allows software developers to create and debug Intel SGX enabled applications in C/C++
- Application code can be put into an enclave via special instructions, and software can be made available to developers via the Intel SGX SDK
- SGX applications typically involve two components: a trusted component, executed inside the enclave, and an untrusted component, executed outside. Data that needs to be passed between trusted and untrusted components is copied from and to the enclave because enclave memory cannot be read directly outside of the enclave. The SGX SDK provides a mechanism to enable function calls between the two components. A bridge function at the enclave entry point dispatches calls to the corresponding functions inside the enclave (ECALLS). This allows an enclave to run only certain functions specified by the developers. Similarly, there are functions that reside in the untrusted component which can be invoked inside the enclave to request services from the outside world (OCALLS)

The *Enclave Definition Language* (EDL) specifies which functions are ECALLs (“enclave calls,” the functions that enter the enclave) and which ones are OCALLs (“outside calls,” the calls to untrusted functions from within the enclave)



**Rust-SGX SDK**



*Problem*: SGX enclaves can have memory corruption and can be exploited

*Solution*: enforce better security in SGX enclaves by leveraging Rust’s memory and thread safety guarantees

- Rust-SGX SDK is built upon Intel SGX SDK: It leverages Intel SGX SDK as the foundation that provides a set of full fledged SGX development APIs, and adds a Rust layer on top of it
- With Rust-SGX, enclave programmers can develop their programs in pure Rust, and Rust-SGX will bridge the gap between the Rust world and Intel SGX interfaces
- The SGX enclave is loaded into the protected memory along with the enclave’s metadata. The SGX enclave binary is linked to Intel SDK libraries, such as libsgx_tstdc.a. Intel SGX SDK exposes standard C/C++ libraries and Rust-SGX SDK is built on top of these libraries, providing Rust style data structures and APIs to developers.
- Compared to the enclave programs developed by Intel SGX SDK, the enclave programs developed with Rust-SGX are significantly more secure at the application layer thanks to the memory-safe Rust language. At the library layer, they have equivalent security properties due to the same dependency on Intel SGX SDK



**Writing enclaves in Rust**



- The enclave code is in the form of a library; the functions that are to be used by application code must have C interoperability 

- The enclave provides information about how to translate the Rust code to C code that the Intel SGX understands:

- - Enclave.config.xml - TODO
  - Enclave.edl: Define the ECALLS in it
  - Enclave.lds - TODO
  - Enclave_private.pem: RSA private key; used to sign enclave.signed.so; how does this work?
  - x86_64-unknown-linux-sgx.json: architecture details? TODO
  - Xargo.toml: TODO

- An application can use the functions exported by the enclave library by linking it. A Rust application provides a build.rs to TODO. The output of build.rs can be found in app/target/release/build/app*/output. Where does libc*/output come from?

- When an application is compiled, the following files are generated: Enclave_u.c, Enclave_u.h, and compiled into Enclave_u.o These files specify how the app interacts with Intel SGX

- What’s the difference between libenclave.a and enclave.signed.so? Besides one being signed and one being dynamic. Why don’t we create a signed dynamic library in the first place?