use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8887").unwrap();
    println!("Listening on port 8887");
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    let message_to_client = msg.clone();

                    websocket.write_message(message_to_client).unwrap();
                    println!("Received message: {}", msg);
                }
            }
        });
    }
}
