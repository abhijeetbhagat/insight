extern crate insight;
use insight::connection::*;
use insight::response::*;
use insight::MediaType;
// use insight::utils::*;

#[test]
fn test_integration() {
    let stream = RtspConnection::new("rtsp://34.227.104.115/vod/mp4:BigBuckBunny_175k.mov");
    if stream.is_ok() {
        println!("Connected");
    } else {
        println!("{}", stream.unwrap_err());
        return;
    }

    let mut conn = stream.unwrap();
    conn.options();
    let mut output = String::new();
    println!("waiting to read more...");
    let mut options = Options;
    println!("options output \n{}", output);

    output.clear();
    conn.describe();
    let mut describe_response = Describe::new();
    println!("describe output:\n{}", output);

    output.clear();
    conn.setup(MediaType::Audio);
    println!("setup output:\n{}", output);

    output.clear();
    conn.play();
    println!("play output:\n{}", output);

    output.clear();
    let mut output = Vec::with_capacity(1500);
    println!("output len {}\n", output.len());
    loop {
        conn.read_server_stream(&mut output);
        for c in output.iter() {
            println!("{:x?}", c);
        }
    }
}

#[test]
fn test_rtsp_start_all_streams() {
    let mut connection = RtspConnection::new("some url")
        .unwrap()
        .open(MediaType::All);
}

#[test]
fn test_rtsp_client_chain() {
    let mut connection = RtspConnection::new("some url").unwrap();
    connection.describe();
    connection.setup(MediaType::All);
    connection.play();
}
