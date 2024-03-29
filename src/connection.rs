use rtp_packet::*;
use std::io::{BufRead, Read, Write};
use std::io::{BufReader, BufWriter};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::vec::Vec;
use utils::*;

use crate::MediaType;

/// an rtsp connection
#[derive(Debug)]
pub struct RtspConnection {
    stream: TcpStream,
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
    url: String,
    cseq: i32,
    session: String,
    audio_track: String,
    video_track: String,
}

impl RtspConnection {
    /// creates a new `RtspConnection`
    pub fn new<S: Into<String>>(url: S) -> Result<RtspConnection, String> {
        let url = url.into();
        let (server, port) = match parse_rtsp_url(&url) {
            Ok((server, port)) => (server, port),
            Err(e) => return Err(e),
        };
        println!("server {}", server);
        let mut ip = None;
        for addr in format!("{}:{}", server, port)
            .to_socket_addrs()
            .map_err(|_| "could not resolve address")?
        {
            match addr {
                std::net::SocketAddr::V4(addr) => {
                    println!("resolved ip {}", addr);
                    let str_addr = addr.to_string();
                    let v: Vec<_> = str_addr.split(":").collect();
                    let resolved_ip: String = v[0].to_owned();
                    ip = Some(resolved_ip);
                    break;
                }
                std::net::SocketAddr::V6(_) => todo!(),
            }
        }

        if let Some(ip) = ip {
            println!("connecting to server ... waiting upto 5 secs ...");
            if let Ok(stream) = TcpStream::connect_timeout(
                &format!("{}:{}", ip, port).parse().unwrap(),
                Duration::from_secs(5),
            ) {
                //Since self referencing in structs is not supported without TP crates,
                //we can use try_clone on the TcpStream to create multiple references to the same stream
                let stream_out = stream.try_clone().unwrap();
                let stream_in = stream.try_clone().unwrap();
                Ok(RtspConnection {
                    stream,
                    writer: BufWriter::new(stream_out),
                    reader: BufReader::new(stream_in),
                    url: url.into(),
                    cseq: 1,
                    session: String::new(),
                    audio_track: String::new(),
                    video_track: String::new(),
                })
            } else {
                Err("error connecting ...".into())
            }
        } else {
            Err("could not resolve rtsp server ip...".into())
        }
    }

    /// sends a options command
    pub fn options(&mut self) {
        let command = format!(
            "OPTIONS {} RTSP/1.0\r\nCSeq: {}\r\n\r\n",
            self.url, self.cseq
        );
        self.cseq += 1;
        self.send(&command.as_bytes());
    }

    /// sends a describe command
    pub fn describe(&mut self) -> Result<(), std::io::Error> {
        let command = format!(
            "DESCRIBE {} RTSP/1.0\r\nCSeq: {}\r\nAccept: application/sdp\r\n\r\n",
            self.url, self.cseq
        );
        self.cseq += 1;
        self.send(&command.as_bytes());

        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        let mut content_length = 0;
        let mut session_parsed = false;
        let mut audio_section = false;
        let mut content_length_parsed = false;
        let mut buf = String::new();

        while line != "\r\n" {
            if !session_parsed && line.contains("Session") {
                let v: Vec<&str> = line.split(':').collect();
                let v: Vec<&str> = v[1].split(';').collect();
                self.session = v[0].trim_start().parse().unwrap();
                session_parsed = true;
            }
            if !content_length_parsed && line.contains("Content-Length") {
                let v: Vec<&str> = line.split(':').collect();
                content_length = v[1].trim().parse().unwrap();
                content_length_parsed = true;
            }

            if line.contains("m=audio") {
                audio_section = true;
            }

            if line.contains("m=video") {
                audio_section = false;
            }

            if line.contains("a=control") {
                let track = line.split(':').collect::<Vec<&str>>()[1];
                if track != "*" {
                    if audio_section {
                        self.audio_track = track.into();
                    } else {
                        self.video_track = track.into();
                    }
                }
            }

            buf.push_str(&line);
            line.clear();
            self.reader.read_line(&mut line)?;
        }

        let mut vec = vec![0u8; content_length];
        self.reader.read_exact(&mut vec)?;
        buf.push_str(&String::from_utf8(vec).unwrap());
        self.parse_sdp(&buf);
        Ok(())
    }

    fn parse_sdp(&mut self, buf: &str) {
        let mut audio_section = false;
        for line in buf.split("\r\n") {
            if line.contains("m=audio") {
                audio_section = true;
            }

            if line.contains("m=video") {
                audio_section = false;
            }

            if line.contains("a=control") {
                let track = line.split(':').collect::<Vec<&str>>()[1];
                if track != "*" {
                    if audio_section {
                        self.audio_track = track.into();
                    } else {
                        self.video_track = track.into();
                    }
                }
            }
        }
    }

    /// sends a setup command
    pub fn setup(&mut self, media: MediaType) -> Result<(), std::io::Error> {
        match media {
            MediaType::Video => self._setup(self.video_track.clone()),
            MediaType::Audio => self._setup(self.audio_track.clone()),
            MediaType::All => {
                self._setup(self.video_track.clone())?;
                self._setup(self.audio_track.clone())
            }
        }
    }

    fn _setup(&mut self, track: String) -> Result<(), std::io::Error> {
        let command = format!("SETUP {}/{} RTSP/1.0\r\nCSeq: {}\r\nUser-Agent: insight\r\nSession: {}\r\nTransport: RTP/AVP;unicast;interleaved=0-1\r\n\r\n",           self.url, track, self.cseq, self.session
        );
        self.cseq += 1;
        self.send(&command.as_bytes());

        let mut data = vec![0; 1500];
        self.reader.read(&mut data)?;
        Ok(())
    }

    /// sends a play command with the given session
    pub fn play(&mut self) -> Result<(), std::io::Error> {
        let command = format!("PLAY {} RTSP/1.0\r\nCSeq: {}\r\nUser-Agent: insight\r\nSession: {}\r\nRange: npt=0.000-\r\n\r\n", self.url, self.cseq, self.session);
        self.cseq += 1;
        self.send(&command.as_bytes());

        let mut data = vec![0; 1500];
        self.reader.read(&mut data)?;
        Ok(())
    }

    /// convenience method that performs rtsp handshake (including play) internally
    pub fn open(&mut self, media: MediaType) -> Result<(), std::io::Error> {
        self.describe()?;
        self.setup(media)?;
        self.play()
    }

    /// sends data over the underlying socket
    fn send(&mut self, data: &[u8]) {
        self.writer.write(data).unwrap();
        //Flushing is necessary in order to send the data over the TCP stream
        self.writer.flush().unwrap();
    }

    /// reads rtp packets from the incoming stream
    pub fn read_server_stream(&mut self) -> Result<Option<RTPPacket>, std::io::Error> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        if buf[0] == 0x24 {
            //'$' means start of RTP packet
            let len = (buf[2] as u16) << 8 | buf[3] as u16; //combine the last two bytes as length of the packet
            let mut data = vec![0; len as usize];
            self.reader.read_exact(data.as_mut_slice())?;
            let packet = data.as_slice().into();
            // println!("packet: {:?}", packet);
            return Ok(Some(packet));
        }
        Ok(None)
    }

    /// gets session of this connection
    pub fn get_session(&self) -> &str {
        &self.session
    }

    /// gets port of this connection
    pub fn get_port(&self) -> u16 {
        self.stream.local_addr().unwrap().port()
    }
}
