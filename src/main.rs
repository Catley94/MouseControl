use std::net::UdpSocket;
use std::str;
use mouse_rs::{Mouse, types::keys::Keys};
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let cursor_speed = 0.2;
    let socket = UdpSocket::bind("127.0.0.1:9922")?;
    let mouse = Mouse::new();

    let mut last_position = (0, 0);
    let threshold = 5;  // Pixel movement threshold

    println!("Listening on 127.0.0.1:9922...");

    let mut buf = [0; 128];

    loop {
        let (amt, _src) = socket.recv_from(&mut buf)?;

        let received = str::from_utf8(&buf[..amt]).unwrap();
        let coords: Vec<&str> = received.split(',').collect();

        if coords.len() == 2 {
            let x: i32 = coords[0].trim().parse().unwrap();
            let y: i32 = coords[1].trim().parse().unwrap();

            // Only move if the difference is significant
            if (x - last_position.0).abs() > threshold || (y - last_position.1).abs() > threshold {
                let (start_x, start_y) = last_position;
                let steps = 10;
                for i in 1..=steps {
                    let interp_x = start_x + (x - start_x) * i / steps;
                    let interp_y = start_y + (y - start_y) * i / steps;
                    mouse.move_to(interp_x as i32, interp_y as i32).expect("Unable to move mouse");
                    std::thread::sleep(Duration::from_millis(5));
                }
                last_position = (x, y);
            }
        }
    }
}
