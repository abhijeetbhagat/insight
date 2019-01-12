use utils::*;
use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::vec::Vec;
use std::io::{BufReader, BufWriter};
use response::*;

pub struct RtspConnection {
    stream : TcpStream,
    writer : BufWriter<TcpStream>,
    reader : BufReader<TcpStream>,
    url    : String,
    seq    : i32,
    session : u64 //TODO is this needed?
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

    pub fn read_generic<T : Response>(&mut self, response : &mut T, data : &mut String) { 
        response.read(&mut self.reader, data);
    }

    #[deprecated] 
    pub fn read(&mut self, data : &mut String) { 
        println!("read called");
        let mut line = String::new();
        self.reader.read_line(&mut line);
        while line != "\r\n" {
            data.push_str(&line);
            line.clear();
            self.reader.read_line(&mut line);
        }
    }

    #[deprecated]
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
        //let mut vec = vec![0; ];
        //self.reader.read_exact(&mut line);
        while line != "\r\n" {
            data.push_str(&line);
            if line.contains("control:trackID=2") {
                break;
            }
            line.clear();
            let num_bytes = self.reader.read_line(&mut line).unwrap();
            if num_bytes == 0 {
                break;
            }
        }
    }

    pub fn read_server_stream(&mut self, data : &mut Vec<u8>) {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf); 
        if buf[0] == 0x24 { //'$' means start of RTP packet
            let len = (buf[2] as u16) << 8 | buf[3] as u16 ;
            let mut buf = vec![0; len as usize];
            self.reader.read_exact(buf.as_mut_slice());
        }
    }

    pub fn get_session(&self) -> u64 {
        self.session
    }

    pub fn get_port(&self) -> u16 {
        self.stream.local_addr().unwrap().port()
    }

} 

