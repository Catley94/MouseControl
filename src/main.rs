use std::net::UdpSocket;
use std::str;
use mouse_rs::{Mouse, types::keys::Keys};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
struct CursorObjectMessage {
    x: i32,
    y: i32,
    left_click_gesture_distance: f32,
    right_click_gesture_distance: f32,
    num_of_hands: i8
}

fn main() -> std::io::Result<()> {
    // Bind the UDP socket to an address and port
    let socket = UdpSocket::bind("127.0.0.1:9922")?;
    let mouse = Mouse::new();

    let mut last_position = (0, 0);
    let base_speed = 0.5;
    let acceleration_factor = 2.0; // Increase speed when moving faster
    let deceleration_factor = 0.2; // Decrease speed when moving slower
    let fast_threshold = 50;       // Distance threshold for acceleration
    let slow_threshold: i32 = 7;       // Distance threshold for deceleration

    let left_click_gesture_distance_threshold: f32 = 6.0;
    let right_click_gesture_distance_threshold: f32 = 6.0;

    let mut left_click_hold = false;

    let click_delay = Duration::from_millis(250); // 500ms delay between right clicks
    let mut last_click = Instant::now() - click_delay; // Initialize to allow immediate click


    println!("Listening on 127.0.0.1:9922...");

    let mut buf = [0; 128];

    loop {
        // Receive data from the socket
        let (amt, _src) = socket.recv_from(&mut buf)?;

        let received = String::from_utf8_lossy(&buf[..amt]);

        if let Ok(cursor_message) = from_str::<CursorObjectMessage>(&received) {
            println!("Received: {:?}", cursor_message);
            mouse.move_to(cursor_message.x, cursor_message.y).expect("Unable to move mouse");


            // println!("Distance: {}", cursor_message.click_gesture_distance);
            println!("Num of hands: {}", cursor_message.num_of_hands);


            /*
                While:
                    number of hands > 1
                        if left click gesture distance < threshold
                            if mouse timer < 250ms
                                mouse.press(left)
                            else
                                timer still going
                        else
                            mouse.release(left) //Check if mouse is held down first?
                        if right click gesture distance < threshold
                            if mouse timer < 250ms
                                mouse.click(right)
                            else
                                timer still going
                        else
                            nothing
                    number of hands == 1
                        if left click gesture distance < threshold
                            if mouse timer < 250ms
                                mouse.click(left)
                            else
                                timer still going
                        else
                            nothing
                        if right click gesture distance < threshold
                            if mouse timer < 250ms
                                mouse.click(left)
                            else
                                timer still going
                        else
                            nothing
             */

            if cursor_message.num_of_hands > 1 {
                if cursor_message.left_click_gesture_distance < left_click_gesture_distance_threshold {
                    if last_click.elapsed() >= click_delay {
                        if !left_click_hold {
                            left_click_hold = true;
                            mouse.press(&Keys::LEFT).expect("Unable to left click");
                            println!("LEFT PRESS");
                            last_click = Instant::now(); // Reset the timer
                        }
                    } else {
                        println!("Timer still going");
                    }
                } else {
                    if left_click_hold {
                        left_click_hold = false;
                        mouse.release(&Keys::LEFT).expect("Unable to release left click");
                    }
                }

                if cursor_message.right_click_gesture_distance < right_click_gesture_distance_threshold {
                    if last_click.elapsed() >= click_delay {
                        mouse.click(&Keys::RIGHT).expect("Unable to right click");
                        println!("RIGHT CLICK");
                        last_click = Instant::now(); // Reset the timer
                    } else {
                        println!("Timer still going");
                    }
                }


                // left_click_hold = true;
                // mouse.press(&Keys::LEFT).expect("Unable to press");
                // println!("HOLD")
            } else {
                if cursor_message.left_click_gesture_distance < left_click_gesture_distance_threshold {
                    if last_click.elapsed() >= click_delay {
                        mouse.click(&Keys::LEFT).expect("Unable to left click");
                        println!("LEFT CLICK W/ LEFT HAND");
                        last_click = Instant::now(); // Reset the timer
                    } else {
                        println!("Timer still going");
                    }
                }

                if cursor_message.right_click_gesture_distance < right_click_gesture_distance_threshold {
                    if last_click.elapsed() >= click_delay {
                        mouse.click(&Keys::LEFT).expect("Unable to left click");
                        println!("LEFT CLICK W/ RIGHT HAND");
                        last_click = Instant::now(); // Reset the timer
                    } else {
                        println!("Timer still going");
                    }
                }

                // if cursor_message.left_click_gesture_distance < left_click_gesture_distance_threshold {
                //
                //     mouse.click(&Keys::LEFT).expect("Unable to click");
                //     println!("LEFT CLICK")
                // }

                if left_click_hold == true {
                    left_click_hold = false;
                    mouse.release(&Keys::LEFT).expect("Unable to release");
                    println!("RELEASE");
                }
            }
        } else {
            println!("Failed to parse JSON: {}", received);
        }

        // Convert the data to a string and print it
        let received = str::from_utf8(&buf[..amt]).unwrap();
        let coords: Vec<&str> = received.split(',').collect();

        if coords.len() == 2 {
            let x: i32 = coords[0].trim().parse().unwrap();
            let y: i32 = coords[1].trim().parse().unwrap();

            let dx = x - last_position.0;
            let dy = y - last_position.1;
            let distance = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();

            let mut steps = 10;
            let mut step_delay = Duration::from_millis(5);

            // Apply acceleration or deceleration based on the distance
            if distance > fast_threshold as f64 {
                steps = (steps as f64 / acceleration_factor) as i32;
                step_delay = Duration::from_millis((step_delay.as_millis() as f64 / acceleration_factor) as u64);
            } else if distance < slow_threshold as f64 {
                steps = (steps as f64 * deceleration_factor) as i32;
                step_delay = Duration::from_millis((step_delay.as_millis() as f64 / deceleration_factor) as u64);
            }

            // Ensure minimum step count
            if steps < 1 {
                steps = 1;
            }

            for i in 1..=steps {
                let interp_x = last_position.0 + (dx * i / steps);
                let interp_y = last_position.1 + (dy * i / steps);
                mouse.move_to(interp_x as i32, interp_y as i32).expect("Unable to move mouse");
                std::thread::sleep(step_delay);
            }

            last_position = (x, y);
        }
    }
}