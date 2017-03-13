#![feature(custom_attribute)]
#![feature(conservative_impl_trait)]

extern crate tokio_core;
extern crate tokio_curl;
extern crate curl;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate option_constructor_derive;

pub mod objects;
pub mod requests;
pub mod errors;

pub static API: &'static str = "https://authserver.mojang.com";
