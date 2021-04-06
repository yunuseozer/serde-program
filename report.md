**Intel SGX**

- Set of security-related instructions built into the CPU that provide a hardware-based Trusted Execution Environment (TEE)
- Allows a user- or kernel-level application to define (inside its address space) protected memory regions, called ***enclaves***
- Enclave memory cannot be read or written by processes outside the enclave itself regardless of their privilege levels and CPU mode; the only way to call an enclave function is through a new instruction that performs several protection checks
- The CPU protects the enclave by encrypting its memory with an encryption key stored inside the CPU that changes randomly every power cycle. The enclave is decrypted on the fly only within the CPU, and only for code and data within the enclave itself. The CPU limits memory access by enforcing checks on the TLB access and memory address translation, and automatically encrypting data when evicting pages to untrusted memory regions
- The contents of the enclaves and the associated data structures are stored inside the Enclave Page Cache (EPC). There is a hard limit on the EPC size, with typical values being 64 and 128 MB. Depending on the size of each enclave, we can expect between 5 and 20 enclaves to reside simultaneously in memory
- An SGX-enabled application has both a *trusted component* (the enclave) and an *untrusted component* (the rest of the application and its modules). The trusted component should be as small as possible in order to save protected memory and limit the attack surface. Applications should also minimize trusted-untrusted component interaction

Intel SGX application execution flow (image by Intel)

<img align="center" src="https://software.intel.com/content/dam/develop/external/us/en/images/intel-software-guard-extensions-tutorial-intel-sgx-foundation-fig03-658687.png">

- The enclave code is enabled using special instructions and loaded as a dynamic library
- Enclaves are written in native C or C++
- **Attestation** is the process of demonstrating that a specific enclave was established on a platform
  - Local attestation - two enclaves on the same platform authenticate each other
  - Remote attestation - an enclave gains the trust of a remote provider
- **Sealing data** is the process of encrypting data to be written to untrusted memory. The data can be read back in the enclave at a later date and unsealed (decrypted). The encryption keys are derived internally on demand and are not exposed to the enclave
- The key used in data sealing is derived from several inputs such as CPU's key material, the security version number, the enclave signing key used by the developer and the enclave's cryptographic signature. The key policy (MRSINGER or MRENCLAVE) determines which inputs are used
- Applications running inside of SGX must be written to be side channel resistant as SGX does not protect against side channel measurement or observation

**Intel SGX SDK**

- A collection of APIs, libraries, documentation, sample source code, and tools that allows software developers to create and debug Intel SGX enabled applications in C/C++

- Communication between the trusted and untrusted components is done via ECALLS ("enclave calls") and OCALLS ("outside calls"). The *EDL* (Enclave Definition Language) specifies which functions are ECALLs and which ones are OCALLs

- The SDK parses the EDL file and generates a pair of proxy functions (also called bridge functions) for each ECALL and OCALL. Each pair of proxy functions has a trusted half (*EnclaveProject_t.h* and *EnclaveProjct_t.c*) and an untrusted half (*EnclaveProject_u.h* and *EnclaveProject_u.c*)

- The sequence of making an ECALL is shown in the figure (taken from Intel)

  

  <img align="center" src="https://software.intel.com/content/dam/develop/external/us/en/images/enclave-development-part-7-fig-1-704116.png">

- Proxy functions are **pure** C functions

- The proxy functions are responsible for:
  - Marshalling data into and out of the enclave: When parameters are passed as pointers, the data referenced by the pointer must be marshaled into and out of the enclave. Note that when providing a pointer parameter to a function, you *must* specify the direction by the keywords in brackets: [in], [out], or [in, out], respectively
  - Placing the return value of the *real* ECALL or OCALL in an address referenced by a pointer parameter
  - Returning the success or failure of the ECALL or OCALL itself as an **sgx_status_t** value
  
- You must always check the return value of the ECALL itself. Any result other than SGX_SUCCESS indicates that the program did not successfully enter the enclave and the requested function did not run

- Debugging:

  - Only the Intel SGX Debugger can debug enclaves
  - Only debug-mode enclaves can be debugged

**Rust-SGX SDK**

*Problem*: SGX enclaves can have memory corruption and can be exploited

*Solution*: enforce better security in SGX enclaves by leveraging Rust’s memory and thread safety guarantees

- Rust-SGX SDK is built on top of Intel SGX SDK: It leverages Intel SGX SDK as the foundation that provides a set of full fledged SGX development APIs, and adds a Rust layer on top of it
- Rust-SDK provides **secure** bindings between Rust and the C/C++ interface exposed by Intel SGX SDK, allowing programmers to develop their enclaves in pure Rust. 
- The SGX enclave is loaded into the protected memory along with the enclave’s metadata. The SGX enclave binary is linked to Intel SDK libraries, such as libsgx_tstdc.a. Intel SGX SDK exposes standard C/C++ libraries and Rust-SGX SDK is built on top of these libraries, providing Rust style data structures and APIs to developers
- Compared to the enclave programs developed by Intel SGX SDK, the enclave programs developed with Rust-SGX are significantly more secure at the application layer thanks to the memory-safe Rust language. At the library layer, they have equivalent security properties due to the same dependency on Intel SGX SDK
- Rust-SGX fully preserves the SGX functionality without imposing significant performance overheads

**Writing enclaves in Rust**

- The enclave code is provided as a Rust library; the functions that are to be used by application code must have C interoperability
- Untrusted parts:
  - Cargo.toml: Includes the dependencies required by the application from the Rust-SDK, such as **sgx_types** (which defines data structures and function interfaces to Intel SGX) and **sgx_urts** (which provides functionality for applications to manage enclaves). Depending on the application needs, other dependencies may also be included, e.g., sgx_tprotected_fs if the application needs to operate files inside the enclave.
  - build.rs: Outputs the compile variables to *rustc*. In the Rust-SGX example, it helps *rustc* to find the location of sgx sdk, the location of Enclave_u generated by Enclave EDL, and some other linkings against urts library. Besides, under the geode-builder, it also analyses other variables such as crate name, toolchain and local dependencies. Details can be referred to *Build* struct in enclave.rs
  - src/*.rs: The main logic of untrusted code
- Trusted parts:
  - Enclave.config.xml: The enclave configuration file; specifies the user defined parameters of the enclave such as stack and heap space. Note that by default, there's 256 KB stack space per thread, and 1 MB global head space.
  - Enclave.edl: Defines the ECALLS and OCALLS. EDL is a specific file format introduced by Intel, very similar to C heading file. *Edger8r*, a EDL compiler written in OCaml, converts *.edl to (1) Enclave_u.c + Enclave_u.h + (2) Enclave_t.c + Enclave_t.h, which contain proxy functions and marshalling data structure used in ECALL/OCALL operation
  - Enclave.lds: Linker script used to hide unnecessary symbols
  - Enclave_private.pem: The enclave's signing key. Used to generate the signed .so file
  - Cargo.toml: The manifest. The target is a staticlib obtained by compiling the trusted enclave part into an .a file, which is then linked together with serial sgx_* files provided by Intel to generate enclave.so. Finally, sgx_sign uses the private key stored in Enclave_private.pem, the configuration described in Enclave.config.xml and the .so file to generate enclave.signed.so
  - build.rs: Similar to build.rs in Untrusted parts
  - src/*.rs: The main logic of trusted code
  - In addition, the enclave defines:
    - x86_64-unknown-linux-sgx.json: architecture details
    - Xargo.toml

- When the application is compiled, Rust SDK generates the trusted proxy functions (Enclave_t.\*) in the enclave folder, and the untrusted proxy functions (Enclave_u.\*) in the application folder
