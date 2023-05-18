# rs-metasploit
[![Metasploit Rust](https://github.com/parrothacker1/rust-metasploit/actions/workflows/rust.yml/badge.svg)](https://github.com/parrothacker1/rust-metasploit/actions/workflows/rust.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![crates.io](https://img.shields.io/badge/crates-1.1.0-blue)

## Overview
Rust Metasploit is a rust library used to make communication with Metasploit RPC Server.This module uses [reqwest](https://docs.rs/reqwest/0.11.4/reqwest/) and [rmp](https://docs.rs/rmp/0.8.10/rmp/) dependencies for communication.

## Example 
Let's have a glance at a simple code 
```rust
use metasploit::client::Client;
let client=Client::new("127.0.0.1",55552,"user","password",true);
print(client.gettoken());
```

## Installation
In your Cargo.toml add the following
```toml
[dependencies]
rust-metasploit="1.2.0"
```

## How To Use
### How to setup metasploit RPC Server
Metasploit RPC can be setup two ways 
#### With msfrpcd
```
msfrpcd -U <username> -P <password> -p <port> -a <IP Address>
```
For example
```
msfrpcd -U user -P password -p 55552 -a 127.0.0.1
```
#### With msfconsole
```
msf6 > load msgrpc Pass=<password> Username=<username>
```
For example
```
msf6 > load msgrpc Pass=password Username=user
```
When you start metasploit RPC Server from console,the ssl value should be false.Like,
```rust
let client=Client::new("127.0.0.1",55552,"user","password",false);
```
### How to use the library
All details about the library have been written in [docs.rs](https://docs.rs/rust-metasploit/1.2.0/metasploit/)

P.S: The latest update of the library do not have any fixed data types for the functions.So it is recommended to read the documentation before using the library.For more information regarding the datatypes read the [metasploit](https://docs.metasploit.com/api/Msf/RPC/RPC_Base.html) documentation
## v1.0.1
* Removed unwanted parameters of modules
* Changed String parameters to str

# v1.1.0
* Added async functions.

# v1.2.0
* Added both async and blocking feature
* Changed data types from fixed to custom type.

## Thanks To
* [reqwest](https://docs.rs/reqwest/0.11.6/reqwest/)
* [serde](https://docs.rs/serde/1.0.130/serde/)
* [rmp](https://docs.rs/rmp/0.8.10/rmp/)
