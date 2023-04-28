extern crate core;

mod network;
mod scanner_args;

use clap::Parser;
use network::is_port_up;
use scanner_args::PortScannerArgs;
use std::sync::mpsc;
use std::thread;
use crate::network::get_socket_addr;

fn main() {
    let cli = PortScannerArgs::parse();
    let ip = cli.ip;
    let ports = cli.ports;
    let udp = cli.udp;

    let (tx, rx): (mpsc::Sender<u16>, mpsc::Receiver<u16>) = mpsc::channel();

    for port in ports.iter() {
        let tx = tx.clone();
        let ip = ip.clone();
        let port_clone = port.clone();

        let socket_addr = get_socket_addr(&ip, &port_clone);

        let _handle: thread::JoinHandle<()> = thread::spawn(move || {
            let is_up = is_port_up(socket_addr, 5, udp);
            if is_up {
                println!("port {} is up on host: {}", port_clone, ip);
            } else {
                println!("port {} is closed on host: {}", port_clone, ip);
            }

            tx.send(port_clone).unwrap();
        });
    }

    for _ in ports.iter() {
        rx.recv().unwrap();
    }
}
