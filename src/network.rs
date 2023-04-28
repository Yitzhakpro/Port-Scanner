use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
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

pub fn is_tcp_port_up(socket_addr: &SocketAddr, timeout: u64) -> bool {
    match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(timeout)) {
        Ok(mut stream) => {
            match stream.set_read_timeout(Some(Duration::from_secs(timeout))) {
                Ok(_) => {},
                Err(_) => return false,
            }

            match stream.set_write_timeout(Some(Duration::from_secs(timeout))) {
                Ok(_) => {},
                Err(_) => return false,
            }

            match stream.write(&[0u8; 1]) {
                Ok(_) => {},
                Err(_) => return false,
            }

            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {},
                Err(_) => return false,
            }

            match stream.shutdown(Shutdown::Both) {
                Ok(_) => {},
                Err(_) => {},
            }
        }
        Err(_) => return false,
    };

    true
}

pub fn is_port_up(socket_addr: SocketAddr, timeout: u64) -> bool {
    // TODO: later add upd scan in cli params
    let is_open = is_tcp_port_up(&socket_addr, timeout);

    return is_open;
}
