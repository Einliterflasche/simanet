use std::sync::mpsc::Sender;

pub struct IpPacket {
    header: IpHeader,
    data: Vec<u8>
}

struct IpHeader {
    data: [u8; 24]
}

impl IpHeader {
    pub fn source_addr(&self) -> u32 {
        let mut addr = 0;
        for i in 12..15 {
            addr += (self.data[i] as u32) << 8;
        };

        addr
    }
}

pub trait Connectable {
    fn send_ip(&self) -> Sender<IpPacket>;
}

pub struct Host {}

impl Connectable for Host {
    fn send_ip(&self) -> Sender<IpPacket> {
        todo!()
    }
}