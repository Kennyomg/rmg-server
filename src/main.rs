use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

const MAX_MESSAGE_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    {
        let server_socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 34254))?;

        let mut clients: Vec<Client> = vec![];

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; MAX_MESSAGE_SIZE];
            match server_socket.recv_from(&mut buf) {
                Ok((size, origin)) => {
                    let message = String::from_utf8_lossy(&buf[..size]);

                    // Add the sender to the list of clients if they are not already registered
                    if !clients.iter().any(|c| c.address == origin) {
                        let client_address = origin;
                        let name = format!("{}", message.trim_end());

                        println!("'{}' connected from {}", name, origin);
                        let mut connected_clients = vec![];
                        for client in &clients {
                            connected_clients.push(client.name.clone());

                            let _ = server_socket.send_to(
                                format!("'{}' joined the chat", name).as_bytes(),
                                &client.address,
                            );
                        }
                        let _ = server_socket.send_to(
                            format!("You successfully joined as {}", name).as_bytes(),
                            client_address,
                        );

                        if connected_clients.len() != 0 {
                            let _ = server_socket.send_to(
                                format!("People in the chat: {}", connected_clients.join(", "))
                                    .as_bytes(),
                                client_address,
                            );
                        } else {
                            let _ = server_socket.send_to(
                                format!("You're the first to join the chat!").as_bytes(),
                                client_address,
                            );
                        }

                        clients.push(Client {
                            name,
                            address: client_address,
                        });
                    } else {
                        // Forward the message to all connected clients
                        let sender = clients.iter().find(|c| c.address == origin).unwrap();
                        println!(
                            "Received message '{}' from {} @ {}",
                            message.trim_end(),
                            sender.name,
                            origin
                        );
                        for client in &clients {
                            if client.address != sender.address {
                                let _ = server_socket.send_to(
                                    format!("'{}' - {}", message.trim_end(), sender.name)
                                        .as_bytes(),
                                    &client.address,
                                );
                            }
                        }
                    }

                    buf.fill(0);
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    } // the socket is closed here
    Ok(())
}

struct Client {
    name: String,
    address: std::net::SocketAddr,
}
