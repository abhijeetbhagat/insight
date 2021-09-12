use std::io::{BufRead, Read};
pub trait Response {
    //Default implementation for some verbs
    fn read<R: Read + BufRead>(&mut self, reader: &mut R, buf: &mut String) {
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

pub struct Options;
impl Response for Options {}

pub struct Describe {
    session: u64,
    audio_track: String,
    video_track: String,
}

impl Describe {
    pub fn new() -> Describe {
        Describe {
            session: 0,
            audio_track: String::new(),
            video_track: String::new(),
        }
    }

    pub fn get_session(&self) -> u64 {
        self.session
    }
}

impl Response for Describe {
    fn read<R: Read + BufRead>(&mut self, reader: &mut R, buf: &mut String) {
        let mut line = String::new();
        reader.read_line(&mut line);
        let mut content_length = 0;
        let mut session_parsed = false;
        let mut audio_section = false;
        let mut content_length_parsed = false;

        while line != "\r\n" {
            //TODO create structs to represent RTSP responses
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
            reader.read_line(&mut line);
        }

        let mut vec = vec![0u8; content_length];
        reader.read_exact(&mut vec);
        buf.push_str(&String::from_utf8(vec).unwrap());
    }
}

pub struct Setup;
impl Response for Setup {}

pub struct Play;
impl Response for Play {}
