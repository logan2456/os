use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn handle_connection(socket: String) 
{
    let listener = TcpListener::bind(socket).unwrap();
    for stream in listener.incoming() 
    {
        let stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("Received message: {}", message);
        let response = format!("Hello, {}!", message.trim_end());
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
