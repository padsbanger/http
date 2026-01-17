mod http;
mod server;
mod website_handler;

use http::Method;
use http::Request;

use ::std::env;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let server = Server::new(String::from("127.0.0.1:8080"));

    server.run(WebsiteHandler::new(public_path));
}

//
//GET /users?id=10 HTTP/1.1\r\n
//use
