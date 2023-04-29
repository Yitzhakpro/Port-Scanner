extern crate core;

mod network;
mod scanner_args;

use std::sync::mpsc;
use std::thread;
use clap::Parser;
use network::{get_socket_addr, is_port_up};
use scanner_args::PortScannerArgs;


fn main() {
    let cli = PortScannerArgs::parse();
    let ip = cli.ip;
    let ports = cli.ports;
    let udp = cli.udp;
    let output = cli.output;
    let verbose = cli.verbose;

    let mut open_ports: Vec<u16> = Vec::new();

    let (tx, rx): (mpsc::Sender<(u16, bool)>, mpsc::Receiver<(u16, bool)>) = mpsc::channel();

    if verbose {
        println!("Starting port scan on: {}", ip);
    }

    for port in ports.iter() {
        let tx = tx.clone();
        let ip = ip.clone();
        let port_clone = port.clone();

        let socket_addr = get_socket_addr(&ip, &port_clone);

        let _handle: thread::JoinHandle<()> = thread::spawn(move || {
            let is_up = is_port_up(socket_addr, 5, udp);
            if is_up && verbose {
                println!("Port {} is open on {}", port_clone, ip);
            }

            tx.send((port_clone, is_up)).unwrap();
        });
    }

    // collecting opened ports from threads
    for _ in ports.iter() {
        if let Ok((port, is_up)) = rx.recv() {
            if is_up {
                open_ports.push(port);
            }
        }
    }

    if verbose {
        println!("Finished port scanning on: {}", ip);
        println!("Creating output file in: {}", output)
    }

    // TODO: create a result file
    for port in open_ports.iter() {
        println!("Port {} is open on {}", port, ip);
    }
}
