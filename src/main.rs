use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use client::Client;

mod client;

fn main() {
    let first_client = Client::new(3555).unwrap();
    let second_client = Client::new(3556).unwrap();

    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        second_client.send(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3555), "hello world")
    });

    first_client.listen()
}
