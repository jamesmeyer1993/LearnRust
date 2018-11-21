use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::fs;

struct Server {
    connection_count: u32,
    thread_count: u32,
}

impl Server {
    pub fn new() -> Server {

        Server {
            connection_count: 0,
            thread_count: 0
        }
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        // response from a query is 120 plaintext spaces
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let server = Server::new();

    // listen to incoming traffic
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        server.handle_connection(stream);
    }
}
