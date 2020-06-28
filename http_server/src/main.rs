use std::thread;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };
use std::fs;
use log::{info};

fn get_client_address(stream: &TcpStream) -> String {
    let address = match stream.peer_addr() {
        Ok(addr) => format!("{}", addr),
        Err(_) => "unknown".to_owned()
    };

    return address;
}

fn handle_connection(mut stream: TcpStream) { 

    // GET / HTTP/1.0 <CRLF><CRLF>
    /*
    GET / HTTP/1.1
    Host: localhost:8888
    User-Agent: curl/7.70.0
    Accept: *
    */

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    //let request_flat = format!("Request : ", request.replace("\n", " ");

    println!("Request : {}", request);
    //println!("Request : {}", request.replace("\n", " "));

    let requests: Vec<&str> = request.split("\n").collect();
    for request in requests {
        println!("Request : {}", request);
    }

    let request_vector: Vec<&str> = request.split(" ").collect();
    let uri = request_vector[1];
    //println!("uri: {}", uri);

    let www_data = "www";
    
    match fs::read_to_string(format!("{}{}", www_data, uri)) {
        Err(_) => {
            let content = fs::read_to_string("www/error/404.html").unwrap();
            let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);
            stream.write(response.as_bytes()).unwrap();
            info!("GET {} HTTP/1.1 404 NOT FOUND {}", uri, get_client_address(&stream));
        },
        Ok(content) => {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);
            stream.write(response.as_bytes()).unwrap();
            info!("GET {} HTTP/1.1 200 OK {}", uri, get_client_address(&stream));
        }
    };

    //stream.write(response.as_bytes().unwrap());
    stream.flush().unwrap();
}

fn main() {
    env_logger::init();
    //log::set_max_level(LevelFilter::Debug);

    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    info!("HTTP server running ...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {                
                //let address = format!("[address : {}]", get_client_address(&stream));                
                //info!("New client {}", address);

                thread::spawn(move || {
                    handle_connection(stream);
                });

            } Err(e) => {
                info!("Client connection failed : {}", e);
            }
        }
        info!("Waiting another client");
    }
    drop(listener);
}
