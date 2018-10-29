use utils::*;
use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::vec::Vec;
use std::io::{BufReader, BufWriter};

pub struct RtspConnection {
    stream : TcpStream,
    writer : BufWriter<TcpStream>,
    reader : BufReader<TcpStream>,
    url    : String,
    seq    : i32,
    session : u64
}

impl RtspConnection {
    pub fn new(url : String) -> Result<RtspConnection, String> {
        let (server, port) = match parse_rtsp_url(&url) {
            Ok((server, port)) => (server, port),
            Err(e) => return Err(e)
        };

        let mut stream = TcpStream::connect(format!("{}:{}", server, port)).unwrap();
        //Since self referencing in structs is not supported without TP crates,
        //we can use try_clone on the TcpStream to create multiple references to the same stream
        let mut stream_out = stream.try_clone().unwrap();
        let mut stream_in = stream.try_clone().unwrap();
        Ok (
            RtspConnection {
                stream : stream,
                writer : BufWriter::new(stream_out),
                reader : BufReader::new(stream_in),
                url    : url.clone(),
                seq    : 0,
                session : 0
            }
        )
    }

    pub fn send(&mut self, data : &[u8]) { 
        self.writer.write(data).unwrap();
        //Flushing is necessary in order to send the data over the TCP stream
        self.writer.flush().unwrap();
    }

    pub fn read(&mut self, data : &mut String) { 
        //None of the read_to_end, read_to_string work
        //TODO: check if we can refactor this
        println!("read called");
        let mut line = String::new();
        self.reader.read_line(&mut line);
        while line != "\r\n" {
            data.push_str(&line);
            line.clear();
            self.reader.read_line(&mut line);
        }
    }

    pub fn read_sdp(&mut self, data: &mut String) {
        let mut line = String::new();
        self.reader.read_line(&mut line);
        while line != "\r\n" {
            //TODO create structs to represent RTSP responses
            if line.contains("Session") {
                let v : Vec<&str> = line.split(':').collect();
                let v : Vec<&str> = v[1].split(';').collect();
                self.session = v[0].trim_left().parse().unwrap();
            }
            data.push_str(&line);
            line.clear();
            self.reader.read_line(&mut line);
        }
        line.clear();
        self.reader.read_line(&mut line);
        while line != "\r\n" {
            data.push_str(&line);
            line.clear();
            let num_bytes = self.reader.read_line(&mut line).unwrap();
            if num_bytes == 0 {
                break;
            }
        }
    }

    pub fn get_session(&self) -> u64 {
        self.session
    }

} 

