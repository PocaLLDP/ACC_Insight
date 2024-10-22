mod connection;

use std::ptr::null;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use acc_lib::{dismiss, init};

fn main() {
    let (tx, rx) = mpsc::channel();
    let receiver = rx;

    let tcp_thread = thread::spawn(move || {

        let mut phys;
        let mut graph;
        let mut stat;

        loop {
            (phys, graph, stat) = init();

            if phys != null() && graph != null() && stat != null() {
                break;
            }
        }

        unsafe {
            connection::main_loop(&*phys, &*graph, &*stat, "127.0.0.1:8080", receiver);
        }
        dismiss(phys, graph, stat);

    });

    println!("10 seconds before shutting down");
    thread::sleep(Duration::from_secs(10));

    println!("Shutting down thread");
    tx.send(()).unwrap();

    tcp_thread.join().unwrap();

    println!("Program shut down successfully");
}
