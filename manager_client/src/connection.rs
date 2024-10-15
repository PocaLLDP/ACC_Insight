use acc_lib::{SPageFilePhysics, SPageFileGraphics, SPageFileStatic, encode_acc_struct, decode_acc_struct};
use serde::{Serialize, Deserialize};
use socket_lib::{send_data, receive_data, create_listener};
use std::net::{TcpListener, TcpStream};
use std::error::Error;



fn receive_struct<T: for<'a> Deserialize<'a>>(stream: & TcpListener) -> Result<T, Box<dyn Error>> {
    let data = receive_data(stream)?;
    let decoded = decode_acc_struct(&data)?;
    Ok(decoded)
}


pub fn main_loop(&phys: &SPageFilePhysics, &graph: &SPageFileGraphics, &stat: &SPageFileStatic, addr: &str){
    let mut listener = create_listener(&addr);

    *phys = receive_struct(&listener).unwrap();
    *graph = receive_struct(&listener).unwrap();
    *stat = receive_struct(&listener).unwrap();
}