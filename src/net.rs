//! This module contains the implementation for Redis serialization protocol (RESP),
//! along with a client and a server that supports a minimal set of commands from Redis

mod client;
mod command;
mod connection;
mod error;
mod frame;
mod server;
mod shutdown;

pub use client::Client;
pub use error::Error;
pub use server::Server;
