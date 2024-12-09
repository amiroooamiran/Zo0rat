use std::{
    io::{self, Read, Write},
    net::TcpStream
};

const PORT: &str = "8000";


fn main() -> io::Result<()>{
    let server_ip = "127.0.0.1";
    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, PORT))?;
    println!("Connect to the servber");
    let message = "hi this is client";
    stream.write_all(message.as_bytes())?;
    println!("message send: {}", message);

    // recive response from server;
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("response from server {}", response);

    Ok(())
}