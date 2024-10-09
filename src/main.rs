use std::net::UdpSocket;
use clap::Parser;
use hex;
use std::iter::zip;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    /// MAC Address of the machine to send a Magic Packet to
    mac_address: Option<String>,

    /// Broadcast address to send the Magic Packet to
    #[arg(short, long, value_name = "BROADCAST", default_value = "255.255.255.255")]
    broadcast: Option<String>,

    /// Port to send the Magic Packet to
    #[arg(short, long, value_name = "PORT", default_value = "7")]
    port: Option<String>
}

struct Args {
    mac: [u8; 6],
    broadcast: String,
    port: String,
}

fn main() {
    let cli = Cli::parse();

    let args = Args {
        mac: parse_mac(&cli.mac_address.as_ref().unwrap()),
        broadcast: cli.broadcast.unwrap(),
        port: cli.port.unwrap(),
    };

    println!("Send magic packet to {}", &cli.mac_address.unwrap());
    match wake_on_lan(args) {
        Ok(_) => {
            println!("Sent Magic Packet");
        },
        Err(e) => {
            println!("Failed to send magic packet");
            println!("{:?}", e);
        }
    }
}

fn wake_on_lan(args: Args) -> std::io::Result<usize> {
    let address = args.broadcast + ":" + &args.port;
    send_magic_packet(
        &build_magic_packet(args.mac)[0..102],
        &address
    )
}

fn parse_mac(mac_input: &str) -> [u8; 6] {
    let mut mac_return = [0u8; 6];
    let mac_working = mac_input.replace(":", "");
    let mac_working = match hex::decode(mac_working) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to parse MAC Address:");
            println!("{:?}", e);
            std::process::exit(1);
        }
    };
    for i in zip(mac_working.into_iter(), 0..6) {
        mac_return[i.1] = i.0;
    }
    mac_return
}

fn build_magic_packet(mac: [u8; 6]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for _i in 0..6 {
        out.push(0xFF);
    }
    for _i in 0..16 {
        for i in 0..6 {
            out.push(mac[i]);
        }
    }
    out
}

fn send_magic_packet(magic_packet: &[u8], address: &str) -> std::io::Result<usize> {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();
    match socket.send_to(magic_packet, address) {
        Ok(o) => return Ok(o),
        Err(e) => return Err(e)
    };
}