use std::thread;
use std::sync::mpsc::Sender;

use crate::net::{Connectable, IpPacket};

mod net;

use net::Host;

fn main() {
    let hosts: Vec<Host> = vec![Host::new(), Host::new()];

    let sender: Sender<IpPacket> = hosts[0].connect();

    thread::spawn(move || {
        println!("Sending packet");
        sender.send(IpPacket::default()).unwrap()
    }).join().unwrap();

    println!("Hello, world!");
}
