use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};


fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
    }
    print!("{}", str::from_utf8(&buffer[..nbytes])?);
    stream.write_all(&buffer[..nbytes])?;
}