use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::vec::Vec;
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex};

struct RtspConnection {
    stream : TcpStream,
    writer : BufWriter<TcpStream>,
    reader : BufReader<TcpStream>
}

impl RtspConnection {
    fn new(server_with_port : String) -> Option<RtspConnection> {
        let mut stream = TcpStream::connect(server_with_port).unwrap();
        let mut stream_out = stream.try_clone().unwrap();
        let mut stream_in = stream.try_clone().unwrap();
        Some (
            RtspConnection {
                stream : stream,
                writer : BufWriter::new(stream_out),
                reader : BufReader::new(stream_in),
            }
        )
    }

    fn send(&mut self, data : &[u8]) { 
        self.writer.write(data).unwrap();
        //Flushing is necessary in order to send the data over the TCP stream
        self.writer.flush().unwrap();
    }

    fn read(&mut self, data : &mut String) { 
        //None of the read_to_end, read_to_string work
        //TODO: check if we can refactor this
        let mut line = String::new();
        self.reader.read_line(&mut line);
        while line != "\r\n" {
            data.push_str(&line);
            line.clear();
            self.reader.read_line(&mut line);
        }
    }
} 

fn main() {
    let stream = RtspConnection::new(String::from("184.72.239.149:554"));
    if stream.is_some(){
        println!("Connected"); 
    } else {
        println!("Error connecting to the server"); 
    } 

    let mut conn = stream.unwrap(); 
    //the extra CRLF at the end of the write string is needed otherwise; RTSP request will not be recognized otherwise. Wireshark
    //shows TCP as the protocol and not RTSP for this request
    conn.send(b"OPTIONS rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov RTSP/1.0\r\nCSeq: 2\r\n\r\n");
    let mut output = String::new();
    loop {
        println!("reading more");
        conn.read(&mut output);
        println!("{}", output);

        output.clear();
        conn.send(b"DESCRIBE rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov RTSP/1.0\r\nCSeq: 3\r\nAccept: application/sdp\r\n\r\n");
        conn.read(&mut output);
        println!("{}", output);
    }
}
