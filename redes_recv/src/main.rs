extern crate pnet;
extern crate pnet_datalink;

use std::env;
use std::io::{self, Write};
use std::process;

use pnet_datalink::{Channel, NetworkInterface};

use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::MutableEthernetPacket;

fn recv_arp(interface: NetworkInterface) {
    let (_, mut receiver) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    println!("Esperando pacotes");
    loop {
        let buf = receiver.next().unwrap();
        let ethernet = pnet::packet::ethernet::EthernetPacket::new(
            &buf[MutableEthernetPacket::minimum_packet_size()..],
        )
        .unwrap();
        let arp = ArpPacket::new(&buf[MutableEthernetPacket::minimum_packet_size()..]).unwrap();

        pub const TYPEARP: EtherType = EtherType(2054);
        if arp.get_protocol_type() == TYPEARP {
            println!("MAC origem: {}", ethernet.get_source());
            println!("MAC destino: {}", ethernet.get_destination());
            println!("EtherType: {}", arp.get_protocol_type());
            println!("IP: {}", arp.get_sender_proto_addr());
            println!(" ");
        }
    }
}
fn main() {
    let mut args = env::args().skip(1);
    let iface_name = match args.next() {
        Some(n) => n,
        None => {
            writeln!(
                io::stderr(),
                "Para o programa funcionar passe como parametro: <NETWORK INTERFACE>"
            )
            .unwrap();
            process::exit(1);
        }
    };

    let interfaces = pnet_datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == iface_name)
        .unwrap();

    recv_arp(interface)
}
