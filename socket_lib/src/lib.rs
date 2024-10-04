use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod server;

//create listener for server
pub fn create_listener(address: &str) -> TcpListener {
    TcpListener::bind(address).unwrap()
}

//create socket connection for client
fn create_connection(address: &str) -> io::Result<TcpStream> {
    TcpStream::connect(address)
}

//send serialized data over socket
fn send_data(stream: &mut TcpStream, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let size = (data.len() as u64).to_le_bytes();
    stream.write_all(&size)?;

    stream.write_all(data)?;
    Ok(())
}

//receive serialized data from socket
fn receive_data(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut size_buffer = [0u8; 8];
    stream.read_exact(&mut size_buffer)?;
    let size = u64::from_le_bytes(size_buffer) as usize;

    let mut buffer = vec![0u8; size];
    stream.read_exact(&mut buffer)?;
    Ok(buffer)
}+
