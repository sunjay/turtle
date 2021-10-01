use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::battlestate::AttackOutcome;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    AttackCoordinates((u8, u8)),
    AttackResult(AttackOutcome),
}

pub enum ChannelType {
    Server,
    Client(SocketAddr),
    UseListener(TcpListener),
}

pub struct Channel {
    stream: TcpStream,
}

impl Channel {
    pub fn client(socket_addr: SocketAddr) -> Self {
        Self {
            stream: TcpStream::connect(socket_addr).expect("Couldn't connect to the server"),
        }
    }

    pub fn server() -> Self {
        let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to port");
        println!(
            "Listening on port: {}, Waiting for connection..(See help: -h, --help)",
            listener.local_addr().unwrap().port()
        );
        let (stream, _) = listener.accept().expect("Couldn't connect to the client");
        Self { stream }
    }

    pub fn serve_using_listener(listener: TcpListener) -> Self {
        let (stream, _) = listener.accept().expect("Couldn't connect to the client");
        Self { stream }
    }

    pub fn send_message(&mut self, msg: &Message) {
        serde_json::to_writer(&self.stream, &msg).expect("Failed to send message");
    }

    pub fn receive_message(&mut self) -> Message {
        let mut de = serde_json::Deserializer::from_reader(&self.stream);
        Message::deserialize(&mut de).expect("Failed to deserialize message")
    }
}
