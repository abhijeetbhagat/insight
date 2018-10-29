pub mod connection;
pub mod utils;
use connection::*;
use utils::*;

fn main() {
    let stream = RtspConnection::new(String::from("rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov"));
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
        conn.read_sdp(&mut output);
        println!("output:\n{}", output);

        output.clear();
        let setup = format!("SETUP rtsp://184.72.239.149:554/vod/mp4:BigBuckBunny_175k.mov/trackID=2 RTSP/1.0\r\nCSeq: 4\r\nUser-Agent: insight\r\nTransport: RTP/AVP;unicast;client_transport=58854-58855\r\nSession: {}\r\n\r\n", conn.get_session());
        conn.send(setup.as_bytes());
        conn.read(&mut output);
        println!("setup output:\n{}", output);
    }
}
