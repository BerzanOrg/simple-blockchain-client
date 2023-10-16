use std::{
    io::BufRead,
    net::{SocketAddr, UdpSocket},
    thread::{sleep, spawn},
    time::Duration,
};

use crate::message::Message;

pub struct Client {
    socket: UdpSocket,
    others: Vec<SocketAddr>,
}

impl Client {
    pub fn new() -> Result<Client, ()> {
        let socket = UdpSocket::bind("127.0.0.1:5555").map_err(|_| ())?;
        Ok(Client {
            socket,
            others: vec![],
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Client, ()> {
        let socket = UdpSocket::bind("127.0.0.1:4444").map_err(|_| ())?;
        Ok(Client {
            socket,
            others: vec![],
        })
    }

    pub fn join_consensus(&mut self, client_addr: SocketAddr) -> Result<(), ()> {
        let msg = Message::JoinConsensus {};

        self.send_msg(&msg, client_addr)?;

        let mut buf = [0u8; 1000];

        let (msg, from) = self.recv_msg(&mut buf).map_err(|_| ())?;

        if from != client_addr {
            return Err(());
        }

        match msg {
            Message::OtherClients { addrs } => {
                self.others = addrs;
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn recv_msg(&self, buf: &mut [u8]) -> Result<(Message, SocketAddr), ()> {
        match self.socket.recv_from(buf) {
            Ok((len, from)) => Ok((buf[0..len].into(), from)),
            Err(_) => Err(()),
        }
    }

    fn send_msg(&self, msg: &Message, to: SocketAddr) -> Result<(), ()> {
        let buf: Vec<u8> = msg.into();
        self.socket.send_to(&buf, to).map(|_| ()).map_err(|_| ())
    }

    pub fn start(&mut self) {
        let mut buf = [0u8; 1000];

        while let Ok((msg, from)) = self.recv_msg(&mut buf) {
            match msg {
                Message::JoinConsensus {} => {
                    self.others.push(from);

                    let msg = Message::OtherClients {
                        addrs: self.others.clone(),
                    };

                    if self.send_msg(&msg, from).is_err() {
                        eprintln!("couldn't send message to {:#?}", from)
                    }

                    let msg = Message::NewClient { addr: from };

                    for &client in &self.others {
                        if self.send_msg(&msg, client).is_err() {
                            eprintln!("couldn't send message to {:#?}", from)
                        }
                    }
                }
                Message::OtherClients { addrs: _ } => (),
                Message::NewClient { addr } => {
                    self.others.push(addr);
                }
                Message::Error {} => {
                    eprintln!("mistaken message sent by {:#?}", from);
                }
            }
        }
    }
}

#[test]
fn test_client() {
    let mut client = Client::new().unwrap();
    let mut client2 = Client::new_test().unwrap();

    spawn(move || client.start());
    spawn(move || {
        client2
            .join_consensus("127.0.0.1:5555".parse().unwrap())
            .unwrap();
        println!("joined consensus");
    });

    sleep(Duration::from_secs(5));
}
