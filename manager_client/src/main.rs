mod connection;

use acc_lib::{SPageFileGraphics, SPageFilePhysics, SPageFileStatic};

fn main() {
    let mut phys = SPageFilePhysics::default();
    let mut graph = SPageFileGraphics::default();
    let mut stat = SPageFileStatic::default();

    connection::main_loop(&mut phys, &mut graph, &mut stat, "127.0.0.1:8080");
}
