mod client;
mod server;

use std::thread;
use std::time::Duration;

fn main() {
    let server_thread = thread::spawn(|| {
        server::run().unwrap();
    });

    thread::sleep(Duration::from_millis(100));

    let client_thread = thread::spawn(|| {
        client::run().unwrap();
    });

    client_thread.join().unwrap();
    server_thread.join().unwrap();
}
