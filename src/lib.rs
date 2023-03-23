use std::{
    net::Ipv4Addr,
    time::Duration,
};

use icmp_socket::packet::WithEchoRequest;
use icmp_socket::*;
use std::io::{Result,Error,ErrorKind};
use std::thread::sleep;

pub struct Pinger {
    payload: Vec<u8>,
    chcksum: u8,
    sock: IcmpSocket4,
}

impl Pinger {
    pub fn new() -> Result<Self> {
        let payload = vec![0u8,1,2,3,4,5,6,7];
        let chcksum: u8 = payload.iter().sum();
        let mut sock = IcmpSocket4::new()?;
        sock.bind("0.0.0.0".parse::<Ipv4Addr>().unwrap())?;
        sock.set_timeout(Some(Duration::from_secs(1)));
        Ok(Self {
            payload,
            chcksum,
            sock,
        })
    }

    pub fn ping(&mut self, addr: &str) -> Result<()> {
        let addr = addr.parse::<Ipv4Addr>().or(Err(Error::from(ErrorKind::InvalidInput)))?;
        let pkt = Icmpv4Packet::with_echo_request(42, 0, self.payload.clone())?;
        self.sock.send_to(addr, pkt)?;
        loop {
            let (resp, _) = self.sock.rcv_from().or(Err(Error::from(ErrorKind::TimedOut)))?;
            if let Icmpv4Message::EchoReply { identifier, sequence: _, payload } = resp.message {
                if identifier != 42 || payload.iter().sum::<u8>() != self.chcksum {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
                return Ok(());
            }
            sleep(Duration::from_millis(50));
        }
    }
}

