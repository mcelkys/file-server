extern crate actix_files;
extern crate actix_web;
extern crate clap;

mod cli_app;

use cli_app::CliApp;

fn main() {
    let app = CliApp::new();
    app.start_server();
}
