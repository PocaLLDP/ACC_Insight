use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn create_listener(address: &str) -> TcpListener {
    TcpListener::bind(address).unwrap()
}

pub fn new_connection_loop(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection!");

                // On gère chaque client dans un thread séparé
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // Crée un tampon pour recevoir les données
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break; // La connexion a été fermée par le client
            }
            Ok(n) => {
                // Affiche les données reçues du client
                let message = String::from_utf8_lossy(&buffer[0..n]);
                println!("Received: {}", message);

                // Réponse du serveur
                stream.write(b"Message received!").unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
