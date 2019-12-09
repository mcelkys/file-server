use crate::options::ServiceOptions;

use core::str::FromStr;

use clap::App;
use clap::Arg;
use clap::ArgMatches;

pub struct AppOptions<'a> {
    matches: ArgMatches<'a>,
}

// public static functions
impl AppOptions<'_> {
    pub fn new() -> Self {
        let app = App::new("File Server")
            .version("1.3.0")
            .author("Mindaugas Celkys <mindaugas.celkys@gmail.com>")
            .about("Static HTTP file server for web developers")
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .help("Binds HTTP server to a specified port")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false)
                    .default_value("80")
            )
            .arg(
                Arg::with_name("path")
                    .long("path")
                    .help("Sets the root server path")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false)
                    .default_value("/"),
            )
            .arg(
                Arg::with_name("directory")
                    .short("d")
                    .long("directory")
                    .help("Sets the root directory (relative to the current working directory), from which static files are served")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false)
                    .default_value("."),
            )
            .arg(
                Arg::with_name("index-file")
                    .short("i")
                    .long("index-file")
                    .help("Sets an 'index' file to be served at the root path")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false)
                    .default_value("index.html"),
            )
            .arg(
                Arg::with_name("threads")
                    .short("t")
                    .long("threads")
                    .help("Number of threads to use for the HTTP server")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("disable-etag")
                    .long("disable-etag")
                    .help("Disables the usage of ETag")
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("disable-last-modified")
                    .long("disable-last-modified")
                    .help("Disables the usage of Last-Modified")
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("disable-content-disposition")
                    .long("disable-content-disposition")
                    .help("Disables the usage of Content-Disposition header")
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("redirect-to-slash-directory")
                    .long("redirect-to-slash-directory")
                    .help("Redirects to a slash-ended path when browsing a directory")
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("show-files-listing")
                    .long("show-files-listing")
                    .help("Show files listing for directories")
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("backlog")
                    .short("b")
                    .long("backlog")
                    .help("Set the maximum number of pending connections")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            ).arg(
                Arg::with_name("client-timeout")
                    .long("client-timeout")
                    .help("Set server client timeout in milliseconds for first request. Defines a timeout for reading client request header. If a client does not transmit the entire set headers within this time, the request is terminated with the 408 (Request Time-out) error.")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("client-shutdown")
                    .long("client-shutdown")
                    .help("Set server connection shutdown timeout in milliseconds. Defines a timeout for shutdown connection. If a shutdown procedure does not complete within this time, the request is dropped.")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("keep-alive")
                    .long("keep-alive")
                    .help("Set server keep-alive setting")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("max-connections")
                    .long("max-connections")
                    .help("Sets the maximum per-thread number of concurrent connections")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("max-rate")
                    .long("max-rate")
                    .help("Sets the maximum per-thread concurrent connection establish processes")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("address")
                    .short("a")
                    .long("address")
                    .help("Binds HTTP server to a specified IP address")
                    .takes_value(true)
                    .number_of_values(1)
                    .multiple(false)
                    .default_value("127.0.0.1")
            );
        Self {
            matches: app.get_matches(),
        }
    }
}

// public methods
impl AppOptions<'_> {
    pub fn get_str(&self, name: &str) -> &str {
        self.matches.value_of(name).unwrap()
    }
    pub fn get_string(&self, name: &str) -> String {
        self.get_str(name).to_owned()
    }
    pub fn has(&self, name: &str) -> bool {
        self.matches.is_present(name)
    }
    pub fn parse<T: FromStr>(&self, name: &str) -> Option<T> {
        let value = self.matches.value_of(name)?;
        match value.parse::<T>() {
            Ok(number) => Some(number),
            Err(_) => {
                println!("Cannot parse {}. This option will be ignored.", name);
                None
            }
        }
    }
    pub fn address(&self) -> String {
        let ip = self.get_string("address");
        let port = self.get_string("port");
        ip + ":" + &port
    }
    pub fn service_options(&self) -> ServiceOptions {
        ServiceOptions::from_app_options(self)
    }
    pub fn threads(&self) -> Option<usize> {
        self.parse("threads")
    }
    pub fn backlog(&self) -> Option<i32> {
        self.parse("backlog")
    }
    pub fn client_timeout(&self) -> Option<u64> {
        self.parse("timeout")
    }
    pub fn client_shutdown(&self) -> Option<u64> {
        self.parse("client-shutdown")
    }
    pub fn keep_alive(&self) -> Option<usize> {
        self.parse("keep-alive")
    }
    pub fn max_connections(&self) -> Option<usize> {
        self.parse("max-connections")
    }
    pub fn max_rate(&self) -> Option<usize> {
        self.parse("max-rate")
    }
}
