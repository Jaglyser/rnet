use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
}
