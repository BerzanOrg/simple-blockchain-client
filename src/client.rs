use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

/// Client for the blockchain.
pub struct Client {
    socket: UdpSocket,
}

impl Client {
    /// Creates a new client on localhost with given port.
    /// # Usage
    /// ```rs
    /// let client = Client::new(3555).unwrap();
    /// ```
    pub fn new(port: u16) -> Option<Client> {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
        let socket = UdpSocket::bind(addr).ok()?;
        Some(Client { socket })
    }

    /// Listens for incoming messages and prints them.
    pub fn listen(&self) {
        let mut buf = [0; 1000];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((number_of_bytes, sender)) => {
                    let msg_buf = &buf[..number_of_bytes];
                    let msg = String::from_utf8_lossy(msg_buf);

                    println!("sender: {sender}\nmessage:{msg}\n");
                }
                Err(err) => eprintln!("an error occured during receiving message: {}", err.kind()),
            }
        }
    }

    /// Sends given message to given receiver.
    pub fn send(&self, receiver: SocketAddrV4, msg: &str) {
        let buf = msg.as_bytes();
        match self.socket.send_to(buf, receiver) {
            Ok(_) => (),
            Err(err) => eprintln!("an error occured during sending message: {}", err.kind()),
        }
    }
}
