use std::io::{BufRead, Read};
pub trait Response {
    fn read<R:Read + BufRead>(&mut self, reader : &mut R, buf : &mut String); 
}

pub struct Options;
impl Response for Options {
    fn read<R:Read + BufRead>(&mut self, reader : &mut R, buf : &mut String) {
        println!("read called");
        let mut line = String::new();
        reader.read_line(&mut line);
        while line != "\r\n" {
            buf.push_str(&line);
            line.clear();
            reader.read_line(&mut line);
        } 
    }
}

pub struct Describe {
    session : u64
}

impl Describe {
    pub fn new() -> Describe {
        Describe {
            session : 0
        }
    } 

    pub fn get_session(&self) -> u64 {
        self.session
    }
}

impl Response for Describe {
    fn read<R:Read + BufRead>(&mut self, reader : &mut R, buf : &mut String) {
        let mut line = String::new();
        reader.read_line(&mut line);
        let mut content_length = 0;
        while line != "\r\n" {
            //TODO create structs to represent RTSP responses
            if line.contains("Session") {
                let v : Vec<&str> = line.split(':').collect();
                let v : Vec<&str> = v[1].split(';').collect();
                self.session = v[0].trim_left().parse().unwrap();
            }
            if line.contains("Content-Length") {
                let v : Vec<&str> = line.split(':').collect();
                content_length = v[1].trim().parse().unwrap();
            }
            buf.push_str(&line);
            line.clear();
            reader.read_line(&mut line);
        } 

        let mut vec = vec![0u8; content_length];
        reader.read_exact(&mut vec);
        buf.push_str(&String::from_utf8(vec).unwrap());
    }
}

struct Setup;
impl Response for Setup {
    fn read<R:Read + BufRead>(&mut self, reader : &mut R, buf : &mut String) {

    }
}

