# Pinger
Simple library to ping network host by IP address.
### Usage
```
use pinger::Pinger;
use std::time::Duration;

fn main() {
    let pinger = Pinger::new().unwrap().set_timeout(Duration::from_secs(2));
    match pinger.ping("8.8.8.8") {
        Ok(()) => println!("response received"),
        Err(e) => println!("{:?}", e),
    }
}
```
