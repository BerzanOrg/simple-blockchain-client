use std::net::SocketAddr;

pub enum Message {
    /// The message a client sends to another client to join the consensus.
    JoinConsensus {},
    /// The message a client sends to a new client to make it know all the other clients.
    OtherClients { addrs: Vec<SocketAddr> },
    /// The message a client sends to all the other clients to make them know that a new client has joined the consensus.
    NewClient { addr: SocketAddr },
    /// Represents errors.
    Error {},
}

impl From<&[u8]> for Message {
    fn from(buf: &[u8]) -> Self {
        let ty = match buf.get(0) {
            Some(v) => v,
            None => return Message::Error {},
        };

        let buf = &buf[1..];

        match ty {
            0 => Message::JoinConsensus {},
            1 => {
                let mut addrs = vec![];

                for chunk in buf.chunks(6) {
                    let ip: [u8; 4] = match chunk[0..4].try_into() {
                        Ok(v) => v,
                        Err(_) => return Message::Error {},
                    };

                    let port: [u8; 2] = match chunk[4..6].try_into() {
                        Ok(v) => v,
                        Err(_) => return Message::Error {},
                    };

                    addrs.push(SocketAddr::from((ip, u16::from_le_bytes(port))));
                }

                Message::OtherClients { addrs }
            }
            2 => {
                let ip: [u8; 4] = match buf[0..4].try_into() {
                    Ok(v) => v,
                    Err(_) => return Message::Error {},
                };

                let port: [u8; 2] = match buf[4..6].try_into() {
                    Ok(v) => v,
                    Err(_) => return Message::Error {},
                };

                let addr = SocketAddr::from((ip, u16::from_le_bytes(port)));

                Message::NewClient { addr }
            }
            _ => Message::Error {},
        }
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        match value {
            Message::JoinConsensus {} => vec![0],
            Message::OtherClients { addrs } => {
                let mut buf = vec![1]; // add 1 byte type

                // for each address
                for addr in addrs {
                    let addr = match addr {
                        SocketAddr::V4(v) => v,
                        SocketAddr::V6(_) => return vec![],
                    };

                    buf.extend_from_slice(&addr.ip().octets()); // add 4 bytes
                    buf.extend_from_slice(&addr.port().to_le_bytes()); // add 2 bytes
                }

                buf
            }
            Message::NewClient { addr } => {
                let mut buf = vec![2]; // add 1 byte type

                let addr = match addr {
                    SocketAddr::V4(v) => v,
                    SocketAddr::V6(_) => return vec![],
                };

                buf.copy_from_slice(&addr.ip().octets()); // add 4 bytes
                buf.copy_from_slice(&addr.port().to_le_bytes()); // add 2 bytes

                buf
            }
            Message::Error {} => vec![],
        }
    }
}
