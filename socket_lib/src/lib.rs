use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod server;

pub fn create_listener(address: &str) -> TcpListener {
    TcpListener::bind(address).unwrap()
}

pub fn create_connection(address: &str) -> io::Result<TcpStream> {
    TcpStream::connect(address)
}

pub fn send_data(stream: &mut TcpStream, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let size = (data.len() as u64).to_le_bytes();
    stream.write_all(&size)?;

    stream.write_all(data)?;
    Ok(())
}

pub fn receive_data(listener: & TcpListener) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut size_buffer = [0u8; 8];
    listener.read_exact(&mut size_buffer)?;
    let size = u64::from_le_bytes(size_buffer) as usize;

    let mut buffer = vec![0u8; size];
    listener.read_exact(&mut buffer)?;
    Ok(buffer)
}
