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
    println!("File server is running at {}", address);
    HttpServer::new(|| {
        let service = Files::new("/", ".").index_file("index.html");
        App::new().service(service)
    })
    .bind(address)
    .unwrap()
    .run()
    .unwrap();
}
