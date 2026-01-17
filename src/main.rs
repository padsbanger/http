mod http;
mod server;
mod website_handler;

use clap::Parser;
use std::env;

use server::Server;
use website_handler::WebsiteHandler;

#[derive(Parser, Debug)]
#[command(about = "Simple static HTTP server")]
struct Args {
    /// Host address to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(short = 'p', long, default_value_t = 8080)]
    port: u16,

    /// Directory to serve files from
    #[arg(short = 'd', long, default_value = "public")]
    directory: String,
}

fn main() {
    let args = Args::parse();

    let directory = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), Args::parse().directory);

    let server = Server::new(Args::parse().host, Args::parse().port.to_string());

    server.run(WebsiteHandler::new(directory));
}
