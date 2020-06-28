use std::thread;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream, Shutdown };
use std::str::from_utf8;
//use std::env;

fn get_client_address(stream: &TcpStream) -> String {
    let address = match stream.peer_addr() {
        Ok(addr) => format!("{}", addr),
        Err(_) => "unknown".to_owned()
    };

    return address;
}

fn get_help() -> String {
    let resutl = String::from("Accepted commands : help ping quit\n");

    return resutl;
}

fn handle_connection(mut stream: TcpStream) { 
    let mut buffer = [0 as u8; 50];

    stream.write("Welcome !!\n".as_bytes()).unwrap();
    stream.write(get_help().as_bytes()).unwrap();
    
    while match stream.read(&mut buffer) {
        Ok(size) => {  
            
            /*if size > 2 {
                let message = &buffer[0..size];
                let text = from_utf8(message).unwrap().trim();
                let response = ["Response : ", text, "\n"].concat();
                let address = get_client_address(&stream);

                println!("Request from client : address -> {} size -> {} request -> {} ", address, size, text);

                stream.write(response.as_bytes()).unwrap();
            }*/

            if size > 2 {
                let message = &buffer[0..size];
                let text = from_utf8(message).unwrap().trim().to_lowercase();

                match text.as_str() {
                    "help" => {
                        stream.write(get_help().as_bytes()).unwrap();
                    },
                    "ping" => {
                        stream.write("pong\n".as_bytes()).unwrap();
                    },
                    "quit" => {
                        stream.write("bye\n".as_bytes()).unwrap();
                        return;
                    },
                    _ => {
                        let response = ["Unknown command : ", &text, "\n"].concat();
                        stream.write(response.as_bytes()).unwrap();
                    }
                };
            }

            stream.flush().unwrap();
            
            true
        } Err(_) => {
            println!("Error, terminating connection {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    /*let args: Vec<String> = env::args().collect();
    let listen_address = match args.len() {
        1 => &args[0],
        _=> "127.0.0.1:8888"
    };

    println!("args : {} {}", args[1], args.len());
    println!("listen_address : {}", listen_address);*/

    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    println!("Tcp server running ...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {                
                let address = format!("[address : {}]", get_client_address(&stream));
                
                println!("New client {}", address);

                thread::spawn(move || {
                    handle_connection(stream);
                });

            } Err(e) => {
                println!("Client connection failed : {}", e);
            }
        }
        println!("Waiting another client");
    }
    drop(listener);
}
