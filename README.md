# Pinger
Simple library to ping network host by IP address.
### Usage
```
use pinger::Pinger;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let pinger = Pinger::new().unwrap().set_timeout(Duration::from_secs(2)).unwrap();
    match pinger.ping("8.8.8.8").await {
        Ok(()) => println!("response received"),
        Err(e) => println!("{:?}", e),
    }
}

```
