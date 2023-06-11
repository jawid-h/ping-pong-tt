use std::net::IpAddr;

use clap::{Parser, Subcommand};
use client::client::{PingClient, PingClientConfig, PingClientConnectionType};
use common::{message::Message, utils::gen_certs::gen_certs};
use server::server::{PongServer, PongServerConfig};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(about = "Run the client and send specified number of Ping! messages")]
    Client {
        #[clap(long, default_value = "127.0.0.1")]
        host: IpAddr,

        #[clap(long, default_value = "4433")]
        port: u16,

        #[clap(long, default_value = "3")]
        ping_count: u32,
    },
    #[clap(about = "Run the server")]
    Server {
        #[clap(long, default_value = "127.0.0.1")]
        host: IpAddr,

        #[clap(long, default_value = "4433")]
        port: u16,

        #[clap(long, default_value = "cert.pem")]
        certificate_path: String,

        #[clap(long, default_value = "key.pem")]
        key_path: String,
    },
    #[clap(about = "Generate certificate files in current working directory")]
    GenCerts,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(SubCommand::Client {
            host,
            port,
            ping_count,
        }) => {
            let ping_client_config = PingClientConfig {
                host: *host,
                port: *port,
                connection_type: PingClientConnectionType::Bidirectional,
                max_retries: 3,
                retry_timeout_millis: 1000,
            };

            let mut ping_client = PingClient::new(ping_client_config);

            let times = if ping_count == &0 {
                None
            } else {
                Some(*ping_count)
            };

            let message = Message::new_request("Ping!".to_string());

            ping_client
                .send_message(&message, times)
                .await
                .expect("sending message failed");
        }
        Some(SubCommand::Server {
            host,
            port,
            certificate_path,
            key_path,
        }) => {
            let pong_server_config = PongServerConfig {
                host: *host,
                port: *port,
                certificate_path: certificate_path.clone(),
                certificate_key_path: key_path.clone(),
            };

            let pong_server = PongServer::new(pong_server_config);

            pong_server.serve().await.expect("Server failed");
        }
        Some(SubCommand::GenCerts) => {
            gen_certs("cert.pem".to_string(), "key.pem".to_string())
                .expect("failed to generate certs");
        }
        None => {}
    }
}
