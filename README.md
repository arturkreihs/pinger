# Pinger
Simple library to ping network host by IP address.
### Usage
```
use pinger::Pinger;

fn main() {
    let mut pinger = Pinger::new().unwrap();
    match pinger.ping("8.8.8.8") {
        Ok(()) => println!("response received"),
        Err(e) => println!("{:?}", e),
    }
}
```
