use acc_lib::{SPageFilePhysics, SPageFileGraphics, SPageFileStatic, encode_acc_struct, decode_acc_struct};
use serde::{Serialize, Deserialize};
use socket_lib::{send_data, receive_data, create_connection};
use std::net::TcpStream;
use std::error::Error;

fn send_struct<T: Serialize>(stream: &mut TcpStream, structure: &T) -> Result<(), Box<dyn Error>> {
    let encoded = encode_acc_struct(structure);
    send_data(stream, &encoded)?;
    Ok(())
}
pub fn main_loop(&phys: &SPageFilePhysics, &graph: &SPageFileGraphics, &stat: &SPageFileStatic, addr: &str){
    let mut stream = create_connection(addr).unwrap();

    send_struct(&mut stream, *phys).unwrap();
    send_struct(&mut stream, *graph).unwrap();
    send_struct(&mut stream, *stat).unwrap();
}