use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for steam in listener.incoming() {
        let mut stream = steam.unwrap();

        println!("连接成功");

        let mut buffers = [0u8; 1024];

        stream.read(&mut buffers).unwrap();

        println!("客户端消息：{}", str::from_utf8(&buffers).unwrap());

        stream.write(&buffers);
    }
}
