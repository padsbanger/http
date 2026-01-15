struct Server {
    address: String,
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }
    fn run(&self) {
        println!("Listening on address:{}", self.address)
    }
}

fn main() {
    let server = Server::new(String::from("127.0.1.1:8080"));

    server.run();
}

//
//GET /users?id=10 HTTP/1.1\r\n
//use
