use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub fn get_socket_addr(ip: &str, port: &u16) -> SocketAddr {
    let full_host: String = format!("{}:{}", ip, port);
    let socket_addr = full_host
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    return socket_addr;
}

// TODO: make it better
pub fn is_udp_port_up(socket_addr: &SocketAddr, timeout: u64) -> bool {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket.set_read_timeout(Some(std::time::Duration::from_secs(timeout))).ok();

    let message = b"check";
    if let Ok(_) = socket.send_to(message, socket_addr) {
        let mut buffer = [0u8; 1];
        match socket.recv(&mut buffer) {
            Ok(_) => true,
            Err(_) => false,
        }
    } else {
        false
    }
}

pub fn is_tcp_port_up(socket_addr: &SocketAddr, timeout: u64) -> bool {
    match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(timeout)) {
        Ok(mut stream) => {
            match stream.set_read_timeout(Some(Duration::from_secs(timeout))) {
                Ok(_) => {}
                Err(_) => return false,
            }

            match stream.set_write_timeout(Some(Duration::from_secs(timeout))) {
                Ok(_) => {}
                Err(_) => return false,
            }

            match stream.write(&[0u8; 1]) {
                Ok(_) => {}
                Err(_) => return false,
            }

            // can be open
            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {}
                Err(_) => {}
            }

            match stream.shutdown(Shutdown::Both) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => return false,
    };

    true
}

pub fn is_port_up(socket_addr: SocketAddr, timeout: u64, udp_scan: bool) -> bool {
    if udp_scan {
        return is_udp_port_up(&socket_addr, timeout);
    }

    return is_tcp_port_up(&socket_addr, timeout);
}
