use std::{
    env,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    str::from_utf8,
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error ocurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1].eq_ignore_ascii_case("server") {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        println!("Listening on port 3333");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || handle_client(stream));
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }
        }
        drop(listener);
    } else if args[1].eq_ignore_ascii_case("client") {
        match TcpStream::connect("localhost:3333") {
            Ok(mut stream) => {
                println!("Connected to server on port 3333");

                let msg = b"Hello!";
                stream.write(msg).unwrap();

                println!("Sent hello! awaiting reply...");
                let mut data = [0 as u8; 6];
                match stream.read(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Response is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    } else {
        println!("Please specify either client or server!")
    }
}
