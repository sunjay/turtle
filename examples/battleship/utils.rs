use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

/// Scans the private port range for an open port and creates a TCP listener
pub fn get_tcp_listener() -> TcpListener {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    for port in 49152..=65535 {
        if let Ok(listener) = TcpListener::bind(SocketAddr::new(localhost, port)) {
            return listener;
        }
    }
    panic!("No ports available!");
}

/// Command-line arguments config
pub enum Opt {
    /// server - multiplayer mode
    Server,
    /// client - multiplayer mode
    Client(SocketAddr),
    /// single player mode
    PlayWithBot,
}

fn print_help() {
    let help = r#"
battleship

USAGE:
    battleship              # no arguments - acts as server
    battleship bot          # single player mode
    battleship <IP:PORT>    # connect to server eg: battleship 192.168.0.120:36354

FLAGS:
    -h, --help       Prints help information

"#;
    println!("{}", help);
}

/// Parses command line arguments
pub fn parse_args() -> Opt {
    let mut args = std::env::args().skip(1);

    match args.next().as_ref().map(String::as_ref) {
        Some("-h") | Some("--help") => {
            print_help();
            std::process::exit(0);
        }
        Some("bot") => Opt::PlayWithBot,
        Some(addr) => {
            let socket_addr = addr.parse().expect("Invalid IP:PORT, See help (-h, --help)");
            Opt::Client(socket_addr)
        }
        None => Opt::Server,
    }
}
