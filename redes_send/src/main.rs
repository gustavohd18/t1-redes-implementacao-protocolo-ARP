extern crate pnet;
extern crate pnet_datalink;

use std::env;
use std::io::{self, Write};
use std::net::{AddrParseError, IpAddr, Ipv4Addr};
use std::process;

use pnet_datalink::{Channel, MacAddr, NetworkInterface};

use pnet::packet::arp::MutableArpPacket;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::{MutablePacket, Packet};

fn send_arp(interface: NetworkInterface, target_ip: Ipv4Addr) {
    let source_ip = interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
        .unwrap();

    let (mut sender, _) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("channel type error"),
        Err(e) => panic!("Error {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac_address());
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Arp);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(interface.mac_address());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    sender
        .send_to(ethernet_packet.packet(), None)
        .unwrap()
        .unwrap();

    println!("Pacote enviado");
}

fn main() {
    let mut args = env::args().skip(1);
    let iface_name = match args.next() {
        Some(n) => n,
        None => {
            writeln!(
                io::stderr(),
                "Para o programa funcionar passe como parametro: <NETWORK INTERFACE> e <TARGET IP>"
            )
            .unwrap();
            process::exit(1);
        }
    };
    let target_ip: Result<Ipv4Addr, AddrParseError> = match args.next() {
        Some(n) => n.parse(),
        None => {
            writeln!(
                io::stderr(),
                "Para o programa funcionar passe como parametro: <NETWORK INTERFACE> e <TARGET IP>"
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
    let _source_mac = interface.mac_address();

    send_arp(interface, target_ip.unwrap());
}
