extern crate diesel;

pub mod apps;
pub mod container;
pub mod core;
pub mod diesel_impl;
pub mod entity;
pub mod users;

pub mod routeguide {
    tonic::include_proto!("routeguide");
}
