#[macro_use]
extern crate diesel;

pub mod core;
pub mod users;
pub mod diesel_impl;
pub mod apps;
pub mod container;

pub mod routeguide {
    tonic::include_proto!("routeguide");
}
