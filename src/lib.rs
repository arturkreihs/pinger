use std::{net::Ipv4Addr, time::Duration};

use icmp_socket::packet::WithEchoRequest;
use icmp_socket::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PingerError {
    #[error("invalid response received")]
    InvalidResponse,

    #[error("invalid address")]
    InvalidAddress(#[from] std::net::AddrParseError),

    #[error("I/O")]
    IO(#[from] std::io::Error),

    #[error("creating ICMP packet")]
    PktCreation,
}

pub struct Pinger {
    payload: Vec<u8>,
    sock: Arc<RwLock<IcmpSocket4>>,
}

impl Pinger {
    pub fn new() -> Result<Self, PingerError> {
        let mut sock = IcmpSocket4::new()?;
        sock.bind("0.0.0.0".parse::<Ipv4Addr>()?)?;
        // sock.set_timeout(Some(Duration::from_secs(1)));
        Ok(Self {
            payload: vec![0u8],
            sock: Arc::new(RwLock::new(sock)),
        })
    }

    pub fn set_timeout(self, dur: Duration) -> Result<Self, PingerError> {
        self.sock.write().unwrap().set_timeout(Some(dur));
        Ok(self)
    }

    pub async fn ping(&self, addr: &str) -> Result<(), PingerError> {
        let addr = addr.parse::<Ipv4Addr>()?;
        let pkt = Icmpv4Packet::with_echo_request(42, 0, self.payload.clone())
            .map_err(|_| PingerError::PktCreation)?;
        let sock = Arc::clone(&self.sock);
        let future = tokio::task::spawn_blocking(move || {
            let mut sock = sock.write().unwrap();
            sock.send_to(addr, pkt)?;
            loop {
                let (resp, _) = sock.rcv_from()?;
                if let Icmpv4Message::EchoReply {
                    identifier: id,
                    sequence: _,
                    payload: pd,
                } = resp.message
                {
                    if id != 42 || pd.first() != Some(&0) {
                        return Err(PingerError::InvalidResponse);
                    }
                    return Ok(());
                }
                sleep(Duration::from_millis(50));
            }
        });
        future.await.map_err(|_| PingerError::InvalidResponse)?
    }
}
