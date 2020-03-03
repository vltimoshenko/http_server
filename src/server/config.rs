use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use num_cpus;

const CONFIG_PATH: &str =  "/etc/httpd.conf";

pub struct Config {
    pub document_root: String,
    pub thread_limit: usize,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            document_root: "/var/www/html".to_string(),
            thread_limit: num_cpus::get(),
        };

        let file = File::open(CONFIG_PATH).unwrap();
        let buf_file = BufReader::new(&file);
        let mut params: HashMap<String, String> = HashMap::new();
        for line in buf_file.lines() {
            let mut l = line.as_ref().unwrap().split_whitespace();
            params.insert(l.next().unwrap_or_default().to_owned(), l.next().clone().unwrap_or_default().to_owned());
        }

        if params.contains_key("thread_limit") {
            config.thread_limit = params.get("thread_limit").unwrap().parse().unwrap()
        }

        if params.contains_key("document_root") {
            config.document_root = params.get("document_root").unwrap().parse().unwrap()
        }

        config
    }

}
