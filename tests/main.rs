extern crate insight;
use insight::connection::*;
use insight::response::*;
use insight::utils::*;

#[test]
fn test_integration() {
    let stream = RtspConnection::new(String::from(
        "rtsp://184.72.239.149/vod/mp4:BigBuckBunny_175k.mov",
    ));
    if stream.is_ok() {
        println!("Connected");
    } else {
        println!("Error connecting to the server");
    }

    let mut conn = stream.unwrap();
    conn.options();
    let mut output = String::new();
    println!("waiting to read more...");
    //conn.read(&mut output);
    let mut options = Options;
    conn.read_generic(&mut options, &mut output);
    println!("options output \n{}", output);

    output.clear();
    conn.describe();
    let mut describe_response = Describe::new();
    conn.read_generic(&mut describe_response, &mut output);
    println!("describe output:\n{}", output);

    output.clear();
    conn.setup();
    conn.read_generic(&mut Setup, &mut output);
    println!("setup output:\n{}", output);

    output.clear();
    conn.play(describe_response.get_session());
    conn.read_generic(&mut Play, &mut output);
    println!("play output:\n{}", output);

    output.clear();
    let mut output = Vec::with_capacity(1500);
    println!("output len {}\n", output.len());
    loop {
        conn.read_server_stream(&mut output);
        //println!("server stream output:\n{}", std::str::from_utf8(&output).unwrap());
        for c in output.iter() {
            println!("{:x?}", c);
        }

        //break;
    }
}
