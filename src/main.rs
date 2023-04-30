extern crate core;

mod network;
mod scanner_args;

use clap::Parser;
use network::{get_socket_addr, is_port_up};
use scanner_args::PortScannerArgs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::{fs, thread};

fn main() {
    let cli = PortScannerArgs::parse();
    let ip = cli.ip;
    let ports = cli.ports;
    let udp = cli.udp;
    let output = cli.output;
    let verbose = cli.verbose;

    let mut open_ports: Vec<u16> = Vec::new();

    let (tx, rx): (mpsc::Sender<(u16, bool)>, mpsc::Receiver<(u16, bool)>) = mpsc::channel();

    println!("Starting port scan on: {}", ip);

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

    // print results
    println!("\nFinished port scanning on: {}", ip);
    println!("Open ports:");
    for port in open_ports.iter() {
        println!("Port {} is open on {}", port, ip);
    }

    // creating output file
    if output.is_some() {
        let output_param = output.unwrap();

        // getting path and creating non-existing directories
        let output_path = Path::new(&output_param);
        if let Some(parent_dir) = output_path.parent() {
            fs::create_dir_all(parent_dir).unwrap();
        }
        let mut output_file = match File::create(&output_path) {
            Err(why) => panic!("Could not create output file, because {}", why),
            Ok(file) => file,
        };

        for port in open_ports.iter() {
            let line = format!("Port {} is open on {}\n", port, ip);
            match output_file.write(line.as_bytes()) {
                Err(_why) => println!("Could write: {} to the output file", line),
                Ok(_) => {}
            }
        }

        if verbose {
            println!("Created output file in {}", output_param);
        }
    }
}
