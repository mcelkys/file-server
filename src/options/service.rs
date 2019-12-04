use crate::options::AppOptions;

#[derive(Clone)]
pub struct ServiceOptions {
    path: String,
    directory: String,
    index_file: String,
    disable_etag: bool,
    disable_last_modified: bool,
    disable_content_disposition: bool,
    redirect_to_slash_directory: bool,
    show_files_listing: bool,
}

// public static functions
impl ServiceOptions {
    pub fn from_app_options(options: &AppOptions) -> Self {
        Self {
            path: options.get_string("path"),
            directory: options.get_string("directory"),
            index_file: options.get_string("index-file"),
            disable_etag: options.has("disable-etag"),
            disable_last_modified: options.has("disable-last-modified"),
            disable_content_disposition: options.has("disable-content-disposition"),
            redirect_to_slash_directory: options.has("redirect-to-slash-directory"),
            show_files_listing: options.has("show-files-listing"),
        }
    }
}

// public methods
impl ServiceOptions {
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
    pub fn directory(&self) -> &str {
        self.directory.as_str()
    }
    pub fn index_file(&self) -> &str {
        self.index_file.as_str()
    }
    pub fn disable_etag(&self) -> bool {
        self.disable_etag
    }
    pub fn disable_last_modified(&self) -> bool {
        self.disable_last_modified
    }
    pub fn disable_content_disposition(&self) -> bool {
        self.disable_content_disposition
    }
    pub fn redirect_to_slash_directory(&self) -> bool {
        self.redirect_to_slash_directory
    }
    pub fn show_files_listing(&self) -> bool {
        self.show_files_listing
    }
}
