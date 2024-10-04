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
                last_packet_id = (*phys).packet_id;
                println!("{}", (*phys).packet_id);
                println!("{}", (*graph).active_cars);
                println!("{:?}", (*stat).ac_version);
                thread::sleep(Duration::from_millis(250));
            }
        }
    }
    dismiss(phys, graph, stat);
}
