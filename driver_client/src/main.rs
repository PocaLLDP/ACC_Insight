mod connection;

use std::ptr::null;
use std::thread;
use std::time::Duration;

use acc_lib::{dismiss, init};

fn main() {
    let mut phys = null();
    let mut graph = null();
    let mut stat = null();

    loop {
        (phys, graph, stat) = init();

        if phys != null() && graph != null() && stat != null() {
            break;
        }
    }
    let mut last_packet_id = unsafe { (*phys).packet_id };

    loop {
        unsafe {
            if last_packet_id == (*phys).packet_id {
                println!("in menu");
                thread::sleep(Duration::from_millis(500));
            } else {
                connection::main_loop(&phys, &graph, &stat, "127.0.0.1:8080");
                break
            }
        }
    }
    dismiss(phys, graph, stat);
}
