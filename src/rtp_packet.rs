struct RTPPacket {
    version : u8,
    padding : bool,
    extension : bool,
    cc : u8,
    marker : bool,
    payload_type : u8,
    seq_num : u16,
    timestamp : u32,
    ssrc : u32
}

impl RTPPacket {
    fn new(version : u8, padding : bool, extension : bool, cc : u8, marker : bool, payload_type : u8, seq_num : u16, timestampj : u32, ssrc : u32) -> RTPPacket {
        RTPPacket {
            version : version,
            padding : padding,
            extension : extension,
            cc : cc,
            marker : marker,
            payload_type : payload_type,
            seq_num : seq_num,
            timestamp : timestamp,
            ssrc : ssrc
        }

    }
}
