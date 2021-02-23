// use toml::value::{Array, Table};

// #[macro_use]
// extern crate lazy_static;

// use std::collections::HashMap;


/**
*
[package]
name    = "tomlread"
version = "0.1.0"
authors = ["jeff.lee <jeff.lee@cloudbases.io>"]
edition = "2018"
[addition]
[content]
[extend]
*
*/
// #[derive(Deserialize)]
// #[derive(Debug)]
// struct Packageinfo {
//     name : String,
//     version : String,
//     authors : Array,
//     edition : String,
// }
// #[derive(Deserialize)]
// #[derive(Debug)]
// struct TomlRegistry {
//     package  : Packageinfo,
//     addition : Table,
//     content  : Table,
//     extend   : Table,
// }

// lazy_static! {
//     static ref VEC:Vec<u8> = vec![0x18u8, 0x11u8];
//     static ref MAP: HashMap<u32, String> = {
//         let mut map = HashMap::new();
//         map.insert(18, "hury".to_owned());
//         map
//     };
//     static ref PAGE:u32 = mulit(18);
// }