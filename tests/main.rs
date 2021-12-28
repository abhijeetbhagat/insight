extern crate insight;
use insight::connection::*;
use insight::MediaType;
// use insight::utils::*;

#[test]
fn test_integration() -> Result<(), std::io::Error> {
    let stream =
        // RtspConnection::new("rtsp://wowzaec2demo.streamlock.net/vod/mp4:BigBuckBunny_115k.mov");
        RtspConnection::new("rtsp://127.0.0.1:8554");
    if stream.is_ok() {
        println!("Connected");
    } else {
        println!("{}", stream.unwrap_err());
        return Ok(());
    }

    let mut conn = stream.unwrap();

    println!("sending describe ...");
    conn.describe()?;

    // conn.setup(MediaType::Video)?;
    println!("sending setup for audio ...");
    conn.setup(MediaType::Audio)?;

    println!("sending play ...");
    conn.play()?;

    loop {
        if let Ok(Some(_)) = conn.read_server_stream() {
            println!("recvd pkt");
        }
    }
}
