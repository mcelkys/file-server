use actix_files::Files;
use actix_web::App as WebApp;
use actix_web::HttpServer;
use clap::App as ClapApp;
use clap::Arg;
use clap::ArgMatches;

pub struct CliApp<'a> {
    matches: ArgMatches<'a>,
}

// public static functions
impl<'a> CliApp<'a> {
    pub fn new() -> Self {
        let app = ClapApp::new("File Server")
            .version("1.1.0")
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
            );
        Self {
            matches: app.get_matches(),
        }
    }
}

// public methods
impl<'a> CliApp<'a> {
    pub fn start_server(self) {
        let address = self.get_address();
        let path = self.get_path();
        let directory = self.get_directory();
        let index_file = self.get_index_file();
        let disable_etag = self.disable_etag();
        let disable_last_modified = self.disable_last_modified();
        let disable_content_disposition = self.disable_content_disposition();
        let redirect_to_slash_directory = self.redirect_to_slash_directory();
        let show_files_listing = self.show_files_listing();
        let mut server = HttpServer::new(move || {
            let mut service = Files::new(&path, &directory).index_file(&index_file);
            if disable_etag {
                service = service.use_etag(false);
            }
            if disable_last_modified {
                service = service.use_last_modified(false);
            }
            if disable_content_disposition {
                service = service.disable_content_disposition();
            }
            if redirect_to_slash_directory {
                service = service.redirect_to_slash_directory();
            }
            if show_files_listing {
                service = service.show_files_listing();
            }
            WebApp::new().service(service)
        });
        if let Some(number) = self.get_threads() {
            server = server.workers(number);
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
}

// private methods
impl<'a> CliApp<'a> {
    fn get_string(&self, name: &str) -> String {
        self.matches.value_of(name).unwrap().to_owned()
    }

    fn has(&self, name: &str) -> bool {
        self.matches.is_present(name)
    }

    fn get_address(&self) -> String {
        let port = self.get_string("port");
        String::from("127.0.0.1:") + &port
    }

    fn get_path(&self) -> String {
        self.get_string("path")
    }

    fn get_directory(&self) -> String {
        self.get_string("directory")
    }

    fn get_index_file(&self) -> String {
        self.get_string("index-file")
    }

    fn get_threads(&self) -> Option<usize> {
        let value = self.matches.value_of("threads")?;
        match value.parse::<usize>() {
            Ok(number) => Some(number),
            Err(error) => {
                println!("Cannot parse thread number: {}", error);
                None
            }
        }
    }

    fn disable_etag(&self) -> bool {
        self.has("disable-etag")
    }

    fn disable_last_modified(&self) -> bool {
        self.has("disable-last-modified")
    }

    fn disable_content_disposition(&self) -> bool {
        self.has("disable-content-disposition")
    }

    fn redirect_to_slash_directory(&self) -> bool {
        self.has("redirect-to-slash-directory")
    }

    fn show_files_listing(&self) -> bool {
        self.has("show-files-listing")
    }
}
