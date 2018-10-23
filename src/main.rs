use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::vec::Vec;

fn main() {
    let mut stream = TcpStream::connect("184.72.239.149:554");
    if stream.is_ok(){
        println!("Connected"); 
    } else {
        println!("Error connecting to the server"); 
    } 

    let mut stream = stream.unwrap(); 
    //the extra CRLF at the end of the write string is needed otherwise; RTSP request will not be recognized otherwise. Wireshark
    //shows TCP as the protocol and not RTSP for this request
    stream.write(b"OPTIONS rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov RTSP/1.0\r\nCSeq: 2\r\n\r\n");
    let mut output = [0; 512];
    loop {
        stream.read(&mut output);
        println!("{}", std::str::from_utf8(&output).unwrap());

        stream.write(b"DESCRIBE rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov RTSP/1.0\r\nCSeq: 3\r\nAccept: application/sdp\r\n\r\n")
    }
}
