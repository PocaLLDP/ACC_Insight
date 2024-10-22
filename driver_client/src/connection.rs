use acc_lib::{SPageFilePhysics, SPageFileGraphics, SPageFileStatic, encode_acc_struct};
use serde::{Serialize};
use socket_lib::{send_data, create_connection};
use std::net::TcpStream;
use std::error::Error;
use std::sync::mpsc::Receiver;
use std::thread::sleep;
use std::time::{Duration};

fn send_struct<T: Serialize>(stream: &mut TcpStream, structure: &T) -> Result<(), Box<dyn Error>> {
    let encoded = encode_acc_struct(structure);
    match encoded {
        Ok(encoded) => {
            send_data(stream, &encoded)?;
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}
pub fn main_loop(phys: &SPageFilePhysics, graph: &SPageFileGraphics, stat: &SPageFileStatic, addr: &str, receiver: Receiver<()>) {
    let mut stream = create_connection(addr).unwrap();
    loop {
        if let Ok(_) = receiver.try_recv() {
            println!("Stop signal received, shutting down");
            stream.shutdown(std::net::Shutdown::Both).expect("shutdown call failed");
            break;
        }

        send_struct(&mut stream, phys).unwrap();
        send_struct(&mut stream, graph).unwrap();
        send_struct(&mut stream, stat).unwrap();

        sleep(Duration::from_millis(5));
    }
}
