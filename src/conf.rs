//! Configuration for the server binary

use std::{io, net::IpAddr};

use config::Config;
use serde::Deserialize;
use tokio::net::TcpListener;

use super::storage::bitcask;

/// All configuration
#[derive(Deserialize)]
pub struct Configuration {
    /// Server configuration.
    pub server: ServerConfig,
    /// Bitcask storage configuration.
    pub bitcask: bitcask::Config,
}

/// Server configuration
#[derive(Deserialize)]
pub struct ServerConfig {
    /// The host address.
    pub host: IpAddr,
    /// The port number.
    pub port: u16,
}

impl Configuration {
    /// Get the configuration from file.
    ///
    /// # Panics
    ///
    /// Panics if the configuration file is not valid
    pub fn get() -> Self {
        let conf = Config::builder()
            .add_source(config::File::with_name("conf/base"))
            .add_source(config::Environment::with_prefix("OPAL"))
            .build()
            .unwrap();
        conf.try_deserialize().unwrap()
    }
}

impl ServerConfig {
    /// Bind to the TCP address given by the host and port.
    pub async fn get_listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind(&format!("{}:{}", self.host, self.port)).await
    }
}