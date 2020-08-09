use std::env;
use std::thread;
use std::vec::Vec;
use std::sync::{Arc, Mutex};
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};

const MSG_SIZE: usize = 32;

fn thread_sleep() {
    thread::sleep(std::time::Duration::from_millis(120));
}

fn main() {
    let post = env::var("PORT").unwrap_or_else(|_| "6000".to_string());
    let address = format!("0.0.0.0:{}", post);
    println!("Server running on: {}", address);

    let server = TcpListener::bind(address).expect("Couldn't bind to host");
    server
        .set_nonblocking(true)
        .expect("Error setting server to non blocking");

    // A vector of all the currently connected clients
    let mut clients: Vec<TcpStream> = vec![];
    // A mutex containing a vector of all message strings sent from the clients.
    let msgs_mutex: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    // Main loop for accepting connections
    loop {
        match server.accept() {
            Ok((mut stream, addr)) => {
                println!("Client connected with address: {}", addr);

                // Clone reference.
                let msgs_mutex = msgs_mutex.clone();
                clients.push(stream.try_clone().expect("Couldn't clone client stream"));

                // Spawn a thread for each connected client.
                thread::spawn(move || loop {
                    let mut buffer = vec![0; MSG_SIZE];

                    match stream.read_exact(&mut buffer) {
                        Ok(()) => {
                            let buf = buffer
                                .into_iter()
                                .take_while(|&x| x != 0)
                                .collect::<Vec<_>>();
                            let msg = String::from_utf8(buf).expect("Not a valid UTF8 message.");
                            println!("Received message: {}", msg);
                            let mut messages = msgs_mutex.lock().unwrap();
                            messages.push(msg);
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Client with address {} disconnected.", addr);
                            break;
                        }
                    }

                    // Sleep for a bit before trying to read again.
                    thread_sleep();
                });
            }
            Err(_) => (),
        }

        // Write all messages from the messages vector into all of our client streams.
        let msgs_mutex = msgs_mutex.clone();
        let mut messages = msgs_mutex.lock().unwrap();

        for msg in &*messages {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buffer = msg.clone().into_bytes();
                    buffer.resize(MSG_SIZE, 0);
                    client.write_all(&buffer).map(|_| client).ok()
                })
                .collect::<Vec<TcpStream>>();
        }

        // Clear all messages that've been sent.
        messages.drain(..);
        thread_sleep();
    }
}