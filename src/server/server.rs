use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

use super::config::Config;
use super::request::*;
use super::thread_pool::*;

static CRLF: &'static str = "\r\n";
static HTTP: &'static str = "HTTP/1.1 ";


pub struct Server {
    pub document_root: String,
    pub thread_pool: ThreadPool,
}

impl Server {
    pub fn new() -> Server {
        let cfg = Config::new();
        Server {
            document_root: cfg.document_root,
            thread_pool: ThreadPool::new(cfg.thread_limit),
        }
    }
    #[allow(unused, unused_mut)]
    fn handle_request(document_root: &str, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => return,
            Ok(..) => {}
            Err(_e) => return,
        };

        let buffer = str::from_utf8(&buffer).unwrap();
        let mut split = buffer.split("\r\n");
        let mut request_raw = split.next().unwrap();

        let mut res = create_response_for_request(document_root, request_raw);
        let res = match res {
            Some(r) => r,
            None => return,
        };

        println!("=========================================\n\
        REQUEST: {:?}\n\
        RESPONSE:\n\
        file: {:?} \n\
        status: {:?}\n\
        headers: {:?}\n\
        =========================================", request_raw, res.file, res.status, res.headers);
        res.send(&mut stream)
    }
    pub fn run(&self) {
        let listener = TcpListener::bind("0.0.0.0:80").unwrap();

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(v) => v,
                Err(_) => continue,
            };

            let doc_root = self.document_root.clone();
            self.thread_pool.execute(move || {
                Server::handle_request(doc_root.as_str(), stream);
            });
        }
    }
}


pub struct Response {
    pub headers: Vec<String>,
    pub status: Option<u32>,
    pub file: Option<File>,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: vec![],
            status: None,
            file: None,
        }
    }

    pub fn send(self, ref mut stream: &mut TcpStream) {
        let mut buf = String::new();
        buf.push_str(HTTP);
        buf.push_str(self.status.unwrap().to_string().as_str());
        buf.push_str(" anything");
        buf.push_str(CRLF);
        for h in self.headers {
            buf.push_str(h.as_str());
            buf.push_str(CRLF);
        }
        buf.push_str(CRLF);

        match stream.write(buf.as_bytes()) {
            Ok(_) => {}
            Err(_e) => {
               println!("Error writing to stream");
                return;
            }
        };

        match self.file {
            Some(mut f) => {
                let mut buf = [0; 1024 * 1024];
                let mut n: u64 = 0;
                loop {
                    match f.read(&mut buf).unwrap() {
                        0 => {
                            break;
                        }
                        i => {
                            n += i as u64;
                            match stream.write(&buf[..i]) {
                                Ok(_) => {}
                                Err(_e) => {
                                   println!("Error writing to stream");
                                    return;
                                }
                            };
                            f.seek(SeekFrom::Start(n as u64)).unwrap();
                        }
                    }
                }
            }
            None => {}
        }
        match stream.flush() {
            Ok(_) => {}
            Err(_e) => {
               println!("Error flushing to stream");
                return;
            }
        };
    }
}
