use std::net::Ipv4Addr;

use clap::{Parser, Subcommand};

use opal::{
    net::Client,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    // Setup global `tracing` subscriber
    let subscriber = get_subscriber("opal".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let args = Args::parse();

    let mut client = Client::connect(format!("{}:{}", args.host, args.port)).await?;
    match args.cmd {
        Command::Set { key, value } => {
            client.set(&key, value.into()).await?;
            println!("\"OK\"");
        }
        Command::Get { key } => match client.get(&key).await? {
            Some(val) => match String::from_utf8(val.to_vec()) {
                Ok(s) => println!("\"{}\"", s),
                Err(_) => println!("{:?}", val),
            },
            None => println!("(nil)"),
        },
        Command::Del { keys } => {
            let n_deleted = client.del(&keys).await?;
            println!("(integer) {}", n_deleted);
        }
    }

    Ok(())
}

/// A minimal Redis client
#[derive(Parser)]
#[clap(name = "opal", version, author, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,

    /// The host address of the server
    #[clap(long, default_value = "127.0.0.1")]
    host: Ipv4Addr,

    /// The port number of the server
    #[clap(long, default_value = "6379")]
    port: u16,
}

#[derive(Subcommand)]
enum Command {
    /// Set key's value
    Set {
        #[clap(name = "KEY")]
        key: String,
        #[clap(name = "VALUE")]
        value: String,
    },

    /// Get key's value
    Get {
        #[clap(name = "KEY")]
        key: String,
    },

    /// Delete keys
    Del {
        #[clap(name = "KEY")]
        keys: Vec<String>,
    },
}
