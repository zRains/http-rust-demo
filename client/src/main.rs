use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    stream.write("123".as_bytes());
}
