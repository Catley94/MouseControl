use std::net::UdpSocket;
use std::str;
use mouse_rs::{types::keys::Keys, Mouse};

fn main() -> std::io::Result<()> {
    // Bind the UDP socket to an address and port
    let socket = UdpSocket::bind("127.0.0.1:9922")?;
    let mouse = Mouse::new();

    println!("Listening on 127.0.0.1:9922...");

    let mut buf = [0; 128];

    loop {
        // Receive data from the socket
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Convert the data to a string and print it
        let received = str::from_utf8(&buf[..amt]).unwrap();
        // println!("Received from {}: {}", src, received);

        // Here you can parse the received data into coordinates, e.g., (x, y)
        let coords: Vec<&str> = received.split(',').collect();
        if coords.len() == 2 {
            let x: i32 = coords[0].trim().parse().unwrap();
            let y: i32 = coords[1].trim().parse().unwrap();
            println!("Coordinates: x = {}, y = {}", x, y);
            mouse.move_to(500, 500).expect("Unable to move mouse");
        }
    }
}
