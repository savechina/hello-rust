#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

///database
pub mod database;

///templates
mod templates;

/// rpcs
pub mod services;

/// sequences
pub mod sequences;

fn main_awesome() {
    println!("Hello, Awesome!");
}
