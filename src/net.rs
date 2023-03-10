use std::{sync::mpsc::{Sender, Receiver, channel}, thread, marker};

#[derive(Default)]
pub struct IpPacket {
    header: IpHeader,
    data: Vec<u8>
}

#[derive(Default)]
struct IpHeader {
    data: Vec<u32>
}

impl IpHeader {
    pub fn source_addr(&self) -> u32 {
        let mut addr = 0;
        for i in 12..16 {
            addr += self.data[i] << 8;
        };

        addr
    }

    pub fn dest_addr(&self) -> u32 {
        let mut addr = 0;
        for i in 16..20 {
            addr += self.data[i] << 8;
        };

        addr
    }
}

pub trait Connectable<Prot> {
    fn connect(&self) -> Sender<Prot>;
}

pub struct Host {
    ip_port: Port<IpPacket>
}

struct Port<Prot> {
    transmitter: Sender<Prot>
}

impl<Prot: marker::Send + 'static> Port<Prot> {
    fn new(tx: Sender<Prot>, rx: Receiver<Prot>) -> Port<Prot> {
        thread::spawn(move || {
            rx.recv().unwrap();
            println!("Port recieved ip packet")
        });

        Port {
            transmitter: tx
        }
    } 
}

impl Host {
    pub fn new() -> Host {
        let (tx, rx) = channel::<IpPacket>();
        Host {
            ip_port: Port::new(tx, rx)
        }
    }
}

impl Connectable<IpPacket> for Host {
    fn connect(&self) -> Sender<IpPacket> {
        self.ip_port.transmitter.clone()
    }
}