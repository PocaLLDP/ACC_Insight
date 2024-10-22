use acc_lib::{SPageFilePhysics, SPageFileGraphics, SPageFileStatic, decode_acc_struct};
use serde::{Deserialize};
use socket_lib::{receive_data, create_listener};
use std::net::{TcpStream};
use std::error::Error;

fn receive_struct<T: for<'a> Deserialize<'a>>(stream: &mut TcpStream) -> Result<T, Box<dyn Error>> {
    let data = receive_data(stream)?;
    let decoded = decode_acc_struct(&data)?;
    Ok(decoded)
}

fn handle_client(phys: &mut SPageFilePhysics, graph: &mut SPageFileGraphics, stat: &mut SPageFileStatic, mut stream: TcpStream){
    loop {
        match receive_struct(&mut stream) {
            Ok(received_phys) => *phys = received_phys,
            Err(e) => {
                println!("Error receiving SPageFilePhysics: {}", e);
                break;
            }
        }

        match receive_struct(&mut stream) {
            Ok(received_graph) => *graph = received_graph,
            Err(e) => {
                println!("Error receiving SPageFileGraphics: {}", e);
                break;
            }
        }

        match receive_struct(&mut stream) {
            Ok(received_stat) => *stat = received_stat,
            Err(e) => {
                println!("Error receiving SPageFileStatic: {}", e);
                break;
            }
        }
    }
}

pub fn main_loop(phys: &mut SPageFilePhysics, graph: &mut SPageFileGraphics, stat: &mut SPageFileStatic, addr: &str) {
    let listener = create_listener(&addr);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("new connection");
                handle_client(phys, graph, stat, stream);
            }
            Err(e) => println!("connection error: {:?}", e),
        }
    }
}
