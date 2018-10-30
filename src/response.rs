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

struct Describe;
impl Response for Describe {
    fn read<R:Read + BufRead>(&mut self, reader : &mut R, buf : &mut String) {

    }
}

struct Setup;
impl Response for Setup {
    fn read<R:Read + BufRead>(&mut self, reader : &mut R, buf : &mut String) {

    }
}

