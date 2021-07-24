use std::net::{TcpListener, TcpStream};

use crate::battlestate::AttackOutcome;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    AttackCoordinates((u8, u8)),
    AttackResult(AttackOutcome),
}

pub enum ChannelType<'a> {
    Server,
    Client(&'a str),
    ServeOnPort(u16),
}

pub struct Channel {
    stream: TcpStream,
}

impl Channel {
    pub fn client(ip_port: &str) -> Self {
        Self {
            stream: TcpStream::connect(ip_port).expect("Couldn't connect to the server"),
        }
    }

    pub fn server() -> Self {
        let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to port");
        println!(
            "Listening on port: {}, Waiting for connection..",
            listener.local_addr().unwrap().port()
        );
        let (stream, _) = listener.accept().expect("Couldn't connect to the client");
        Self { stream }
    }

    pub fn serve_on_port(port: u16) -> Self {
        let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).expect("Failed to bind to port");
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
