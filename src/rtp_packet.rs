#[derive(Debug)]
pub struct RTPPacket {
    pub version : u8,
    pub padding : bool,
    pub extension : bool,
    pub cc : u8, //csrc count
    pub marker : bool,
    pub payload_type : u8,
    pub seq_num : u16,
    pub timestamp : u32,
    pub ssrc : u32, //synchronization source identifier
    pub csrcs : Option<Vec<u32>>, //contributing source identifiers
    pub profile_specific_ext_hdr_id : Option<u16>,
    pub ext_hdr_len : Option<u16>
}

impl RTPPacket {
    fn new(version : u8, padding : bool, extension : bool, cc : u8, marker : bool, payload_type : u8, seq_num : u16, timestamp : u32, ssrc : u32, csrcs : Option<Vec<u32>>, profile_specific_ext_hdr_id : Option<u16>, ext_hdr_len : Option<u16>) -> RTPPacket {
        RTPPacket {
            version : version,
            padding : padding,
            extension : extension,
            cc : cc,
            marker : marker,
            payload_type : payload_type,
            seq_num : seq_num,
            timestamp : timestamp,
            ssrc : ssrc,
            csrcs : csrcs,
            profile_specific_ext_hdr_id : profile_specific_ext_hdr_id,
            ext_hdr_len : ext_hdr_len 
        }

    }
}
