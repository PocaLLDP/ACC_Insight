mod connection;

use std::ptr::null;
use socket_lib;

use acc_lib::{SPageFileGraphics, SPageFilePhysics, SPageFileStatic};

fn main() {
    let mut phys: *const SPageFilePhysics = null();
    let mut graph: *const SPageFileGraphics = null();
    let mut stat: *const SPageFileStatic = null();
    connection::main_loop(&phys, &graph, &stat, "127.0.0.1:8080");
}
