use std::thread;
use std::time::Duration;

mod shared_file_out;
mod acc_memory_reader;

use shared_file_out::SPageFilePhysics;
use crate::acc_memory_reader::{dismiss_graph, dismiss_phys, dismiss_stat};

fn main() -> windows::core::Result<()> {
    let phys = acc_memory_reader::init_phys();
    let graph = acc_memory_reader::init_graph();
    let stat = acc_memory_reader::init_stat();

    loop {
        unsafe {
            if phys.is_null() {
                thread::sleep(Duration::from_millis(500));
                continue;
            }
            print!("{}", (*phys).packet_id);
            print!("{}", (*graph).active_cars);
            print!("{:?}", (*stat).ac_version);

        }
    }
    dismiss_phys(phys);
    dismiss_graph(graph);
    dismiss_stat(stat);
}
