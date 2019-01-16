pub struct RTPPacket {
    pub version : u8,
    pub padding : bool,
    pub extension : bool,
    pub cc : u8,
    pub marker : bool,
    pub payload_type : u8,
    pub seq_num : u16,
    pub timestamp : u32,
    pub ssrc : u32
}

impl RTPPacket {
    fn new(version : u8, padding : bool, extension : bool, cc : u8, marker : bool, payload_type : u8, seq_num : u16, timestamp : u32, ssrc : u32) -> RTPPacket {
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
