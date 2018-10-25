pub mod connection;
pub mod utils;
use connection::*;
use utils::*;

fn main() {
    let stream = RtspConnection::new(String::from("184.72.239.149:554"));
    if stream.is_ok(){
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
        println!("waiting to read more...");
        conn.read(&mut output);
        println!("{}", output);

        output.clear();
        conn.send(b"DESCRIBE rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov RTSP/1.0\r\nCSeq: 3\r\nAccept: application/sdp\r\n\r\n");
        conn.read(&mut output);
        println!("{}", output);
    }
}
