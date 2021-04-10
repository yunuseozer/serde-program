# Code example

This is a code example on how library crates can be written that supports both usage in std and sgx.

The library defines a struct called MyImportantObj that has the following fields:

* my_string_value: String
* my_u128_value: u128

MyImportantObj has the following methods:

* pub fn hash() -> [u8; 8]
* pub fn to_json() -> String

The hash() method performs a twox hash on the content of MyImportantObj.

The content of MyImportantObj is defined as follows:

[ my_u128_value : 16 bytes ][ my_string_value : my_string_value.len() bytes ]

Additionally, MyImportantObj is Serializable and Deserializable.