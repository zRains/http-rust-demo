use std::io::Read;
use std::net::TcpListener;

use http::request::HttpRequest;

use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Server<'a> {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();

        println!("Running on: {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request_buffer = [0u8; 256];

            stream.read(&mut request_buffer).unwrap();

            let request: HttpRequest = String::from_utf8(request_buffer.to_vec()).unwrap().into();

            // println!("{:#?}", request);

            Router::route(request, &mut stream);
        }
    }
}
