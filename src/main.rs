extern crate actix_files;
extern crate actix_web;

use std::env::args;

use actix_files::NamedFile;
use actix_web::web::get;
use actix_web::web::Path;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::Result as WebResult;

fn main() {
    let port = match args().nth(1) {
        Some(port) => port,
        None => String::from("80"),
    };
    let address = String::from("127.0.0.1:") + &port;
    println!("File server is running at {}", address);
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(index))
            .route("/{file:.*}", get().to(file))
    })
    .bind(address)
    .unwrap()
    .run()
    .unwrap();
}

fn index() -> WebResult<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

fn file(path: Path<String>) -> WebResult<NamedFile> {
    Ok(NamedFile::open(path.as_str())?)
}
