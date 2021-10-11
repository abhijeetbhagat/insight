extern crate insight;
use insight::connection::*;
use insight::response::*;
use insight::MediaType;
// use insight::utils::*;

#[test]
fn test_integration() {
    let stream =
        RtspConnection::new("rtsp://wowzaec2demo.streamlock.net/vod/mp4:BigBuckBunny_115k.mov");
    if stream.is_ok() {
        println!("Connected");
    } else {
        println!("{}", stream.unwrap_err());
        return;
    }

    let mut conn = stream.unwrap();

    conn.describe();

    conn.setup(MediaType::Video);

    conn.play();

    loop {
        if let Some(_) = conn.read_server_stream() {
            println!("recvd pkt");
        }
    }
}

#[test]
fn test_rtsp_start_all_streams() {
    let mut connection = RtspConnection::new("some url")
        .unwrap()
        .open(MediaType::All);
}

#[ignore]
#[test]
fn test_rtsp_client_chain() {
    let mut connection = RtspConnection::new("some url").unwrap();
    connection.describe();
    connection.setup(MediaType::All);
    connection.play();
}
