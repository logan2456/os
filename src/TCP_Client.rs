use std::net::TcpStream;
use std::io::{self, prelude::*};

fn request(socket: String, message: String) -> String
{
    // 连接到服务器
    let mut stream = TcpStream::connect(socket).unwrap();
    // 发送消息到服务器
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();

    // 创建一个缓冲区接收服务器的响应
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    // 将接收到的数据转换为字符串
    String::from_utf8_lossy(&buffer[..]);
}
