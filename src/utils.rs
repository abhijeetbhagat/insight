pub fn parse_rtsp_url(url: &str) -> Result<(String, String), String> {
    //rtsp://server:port/whatever/
    // wowzaec2demo.streamlock.net
    let v: Vec<&str> = url.splitn(4, '/').collect();
    let v: Vec<&str> = v[2].splitn(1, '/').collect();
    let v: Vec<&str> = v[0].splitn(2, ':').collect();
    Ok((
        String::from(v[0]),
        if v.len() == 2 {
            String::from(v[1])
        } else {
            String::from("554")
        },
    ))
}

#[test]
fn test_parse_rtsp_url_success() {
    assert_eq!(
        parse_rtsp_url(&String::from("rtsp://test:543")).unwrap(),
        (String::from("test"), String::from("543"))
    );
}

#[test]
fn test_parse_rtsp_url_default_port() {
    assert_eq!(
        parse_rtsp_url(&String::from("rtsp://test")).unwrap(),
        (String::from("test"), String::from("554"))
    );
}

#[test]
fn test_parse_rtsp_url_longer_url() {
    assert_eq!(
        parse_rtsp_url(&String::from("rtsp://test/whatever/blah/blah/blah")).unwrap(),
        (String::from("test"), String::from("554"))
    );
}

#[test]
fn test_parse_rtsp_url_longer_url_with_port() {
    assert_eq!(
        parse_rtsp_url(&String::from("rtsp://test:554/whatever/blah/blah/blah")).unwrap(),
        (String::from("test"), String::from("554"))
    );
}

#[test]
fn test_parse_actual_rtsp_url() {
    assert_eq!(
        parse_rtsp_url(&String::from(
            "rtsp://wowzaec2demo.streamlock.net/vod/mp4:BigBuckBunny_115k.mov"
        ))
        .unwrap(),
        (
            String::from("wowzaec2demo.streamlock.net"),
            String::from("554")
        )
    );
}
