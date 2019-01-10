pub mod connection;
pub mod utils;
pub mod response;
use connection::*;
use utils::*;
use response::*;

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
    //loop {
    println!("waiting to read more...");
    //conn.read(&mut output);
    let mut options = Options;
    conn.read_generic(&mut options, &mut output);
    println!("options output \n{}", output);

    output.clear();
    conn.send(b"DESCRIBE rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov RTSP/1.0\r\nCSeq: 3\r\nAccept: application/sdp\r\n\r\n");
    let mut describe_response = Describe::new();
    conn.read_generic(&mut describe_response, &mut output);
    println!("describe output:\n{}", output);

    output.clear();
    let setup = format!("SETUP rtsp://184.72.239.149:554/vod/mp4:BigBuckBunny_175k.mov/trackID=2 RTSP/1.0\r\nCSeq: 4\r\nUser-Agent: insight\r\nTransport: RTP/AVP;unicast;interleaved=0-1\r\n\r\n");
    conn.send(setup.as_bytes());
    conn.read_generic(&mut Setup, &mut output);
    println!("setup output:\n{}", output);

    output.clear();
    let play = format!("PLAY rtsp://184.72.239.149:554/vod/mp4:BigBuckBunny_175k.mov/trackID=2 RTSP/1.0\r\nCSeq: 5\r\nUser-Agent: insight\r\nSession: {}\r\nRange: npt=0.000-\r\n\r\n", describe_response.get_session());
    conn.send(play.as_bytes());
    conn.read_generic(&mut Play, &mut output);
    println!("play output:\n{}", output);

    output.clear();
    let mut output = vec![0; 1314];
    println!("output len {}\n", output.len());
    conn.read_server_stream(&mut output);
    println!("server stream output:\n{}", std::str::from_utf8(&output).unwrap());

    //break;
    //}
}
