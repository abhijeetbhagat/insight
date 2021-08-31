use response::*;
use rtp_packet::*;
use std::io::{BufRead, Read, Write};
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::vec::Vec;
use utils::*;

pub struct RtspConnection {
    stream: TcpStream,
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
    url: String,
    seq: i32,
    session: u64, //TODO is this needed?
}

impl RtspConnection {
    pub fn new(url: String) -> Result<RtspConnection, String> {
        let (server, port) = match parse_rtsp_url(&url) {
            Ok((server, port)) => (server, port),
            Err(e) => return Err(e),
        };

        let stream = TcpStream::connect(format!("{}:{}", server, port)).unwrap();
        //Since self referencing in structs is not supported without TP crates,
        //we can use try_clone on the TcpStream to create multiple references to the same stream
        let stream_out = stream.try_clone().unwrap();
        let stream_in = stream.try_clone().unwrap();
        Ok(RtspConnection {
            stream,
            writer: BufWriter::new(stream_out),
            reader: BufReader::new(stream_in),
            url: url.clone(),
            seq: 0,
            session: 0,
        })
    }

    pub fn send(&mut self, data: &[u8]) {
        self.writer.write(data).unwrap();
        //Flushing is necessary in order to send the data over the TCP stream
        self.writer.flush().unwrap();
    }

    pub fn read_generic<T: Response>(&mut self, response: &mut T, data: &mut String) {
        response.read(&mut self.reader, data);
    }

    pub fn read_server_stream(&mut self, data: &mut Vec<u8>) {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf);
        loop {
            if buf[0] == 0x24 {
                //'$' means start of RTP packet
                let len = (buf[2] as u16) << 8 | buf[3] as u16; //combile the last two bytes as length of the packet
                                                                //println!("Reading {} bytes\n", len);
                let mut data = vec![0; len as usize];
                self.reader.read_exact(data.as_mut_slice());
                println!("{:?}", self.read_header(&data));
            }
            self.reader.read_exact(&mut buf);
        }
    }

    fn read_header(&self, data: &[u8]) -> RTPPacket {
        let version = if data[0] & 0x80 != 0 { 2 } else { 1 };
        let padding = (data[0] & 0x20) > 0;
        let extension = (data[0] & 0x10) > 0;
        let cc = data[0] & 0xF;
        let marker = (data[1] & 0x80) > 0;
        let payload_type = data[1] & 0x7F;
        let seq_num = (data[2] as u16) << 8 | data[3] as u16;
        //TODO: abhi - create a struct to represent a raw packet and add utility methods to read
        //data like - read_unsigned_int(), read_byte(), etc.
        let timestamp = ((data[4] as u32) << 24)
            | ((data[5] as u32) << 16)
            | ((data[6] as u32) << 8)
            | data[7] as u32;

        let ssrc = ((data[8] as u32) << 24)
            | ((data[9] as u32) << 16)
            | ((data[10] as u32) << 8)
            | data[11] as u32;

        let mut i = 12usize;
        let mut csrcs = Vec::new();
        for _ in 0..cc {
            let csrc = ((data[i] as u32) << 24)
                | ((data[i + 1] as u32) << 16)
                | ((data[i + 2] as u32) << 8)
                | data[i + 3] as u32;
            csrcs.push(csrc);
            i += 4;
        }

        println!("{}", data[12]);
        RTPPacket {
            version,
            padding,
            extension,
            cc,
            marker,
            payload_type,
            seq_num,
            timestamp,
            ssrc,
            csrcs: if cc > 0 { Some(csrcs) } else { None },
            profile_specific_ext_hdr_id: None,
            ext_hdr_len: None,
            payload: data[i..].into(),
        }
    }

    pub fn get_session(&self) -> u64 {
        self.session
    }

    pub fn get_port(&self) -> u16 {
        self.stream.local_addr().unwrap().port()
    }
}
