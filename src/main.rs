extern crate actix_files;
extern crate actix_web;
extern crate clap;

mod options;

use options::AppOptions;

use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;

fn main() {
    let app_options = AppOptions::new();
    let address = app_options.address();
    let service_options = app_options.service_options();
    let mut server = HttpServer::new(move || {
        let mut service = Files::new(service_options.path(), service_options.directory())
            .index_file(service_options.index_file());
        if service_options.disable_etag() {
            service = service.use_etag(false);
        }
        if service_options.disable_last_modified() {
            service = service.use_last_modified(false);
        }
        if service_options.disable_content_disposition() {
            service = service.disable_content_disposition();
        }
        if service_options.redirect_to_slash_directory() {
            service = service.redirect_to_slash_directory();
        }
        if service_options.show_files_listing() {
            service = service.show_files_listing();
        }
        App::new().service(service)
    });
    if let Some(number) = app_options.threads() {
        server = server.workers(number);
    }
    if let Some(number) = app_options.backlog() {
        server = server.backlog(number);
    }
    if let Some(number) = app_options.client_shutdown() {
        server = server.client_shutdown(number);
    }
    if let Some(number) = app_options.client_timeout() {
        server = server.client_timeout(number);
    }
    if let Some(number) = app_options.keep_alive() {
        server = server.keep_alive(number);
    }
    if let Some(number) = app_options.max_connections() {
        server = server.maxconn(number);
    }
    if let Some(number) = app_options.max_rate() {
        server = server.maxconnrate(number);
    }
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
