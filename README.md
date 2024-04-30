## WASM-AES: A Web based data encryption tool using WASM
![MIT licensed][license-image]
![Category][category-image]


WASM-AES is a web based data encryption and decryption tool which runs directly in the user's browser using WASM. The encryption cipher used is AES and the key size is 256 bits.

### Features
- Encrypt and decrypt data using AES-256 GCM and ECB mode.
- PBKDF2-HMAC-SHA256 is used for the key derivation. The default iterations the program use is 60000
- Does the cipher operation in the uses's browser. The User data and key are not transmitted over the web.
- Does not require a server to use this program. Just use it directly by opening the html file, but you need to somehow deal with CORS.
- The encrypted data is presented as base64 because it is convenient.


[//]: # (badges)

[license-image]: https://img.shields.io/badge/Apache_2.0-yellow.svg
[category-image]: https://img.shields.io/badge/category-Data_encryption_software-darkred.svg
