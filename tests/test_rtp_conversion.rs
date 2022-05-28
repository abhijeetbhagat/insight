extern crate insight;
use insight::rtp_packet::RTPPacket;

#[test]
fn test_rtp_conversion() {
    let packet = RTPPacket {
        version: 1,
        padding: false,
        extension: false,
        cc: 1,
        marker: false,
        payload_type: 96,
        seq_num: 1,
        timestamp: 12355,
        ssrc: 32323,
        csrcs: None,
        profile_specific_ext_hdr_id: None,
        ext_hdr_len: None,
        payload: vec![1, 2, 3, 4],
    };
    let raw = Vec::<u8>::from(packet);
    assert!(raw.len() == 16);
}
