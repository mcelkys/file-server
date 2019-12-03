extern crate actix_files;
extern crate actix_web;

use std::env::args;

use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;

fn main() {
    let port = match args().nth(1) {
        Some(port) => port,
        None => String::from("80"),
    };
    let address = String::from("127.0.0.1:") + &port;
    let server = HttpServer::new(|| {
        let service = Files::new("/", ".").index_file("index.html");
        App::new().service(service)
    });
    match server.bind(address.clone()) {
        Ok(server) => {
            println!("Starting file server at {}", address);
            match server.run() {
                Ok(_) => {}
                Err(error) => println!("{}", error),
            }
        }
        Err(error) => println!("{}", error),
    }
}
