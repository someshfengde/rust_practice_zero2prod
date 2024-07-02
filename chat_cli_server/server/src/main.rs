use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "0.0.0.0:6000";
const MSG_SIZE: usize = 32;

fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main(){ 
    let server = TcpListener::bind(LOCAL).expect("Failed to bind to the address");
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![]; 
    let (tx, rx) = mpsc::channel::<String>();
    
    loop{
        if let Ok((mut socket, addr)) = server.accept(){
            println!("Client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));
            thread::spawn(move || loop{
                let mut buffer = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buffer){
                    Ok(_) => {
                        let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send message to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }
    }

    if let Ok(msg) = rx.try_recv(){
        clients = clients.into_iter().filter_map(|mut client| {
            let mut buffer = msg.clone().into_bytes();
            buffer.resize(MSG_SIZE, 0);
            client.write_all(&buffer).map(|_| client).ok()
        }).collect::<Vec<_>>();
        sleep();
    }
}