#[derive(Debug, Clone)]
pub struct RTPPacket {
    pub version: u8,
    pub padding: bool,
    pub extension: bool,
    pub cc: u8, //csrc count
    pub marker: bool,
    pub payload_type: u8,
    pub seq_num: u16,
    pub timestamp: u32,
    pub ssrc: u32,               //synchronization source identifier
    pub csrcs: Option<Vec<u32>>, //contributing source identifiers
    pub profile_specific_ext_hdr_id: Option<u16>,
    pub ext_hdr_len: Option<u16>,
    pub payload: Vec<u8>,
}

impl RTPPacket {
    fn new(
        version: u8,
        padding: bool,
        extension: bool,
        cc: u8,
        marker: bool,
        payload_type: u8,
        seq_num: u16,
        timestamp: u32,
        ssrc: u32,
        csrcs: Option<Vec<u32>>,
        profile_specific_ext_hdr_id: Option<u16>,
        ext_hdr_len: Option<u16>,
    ) -> RTPPacket {
        RTPPacket {
            version,
            padding,
            extension,
            cc,
            marker,
            payload_type,
            seq_num,
            timestamp,
            ssrc,
            csrcs,
            profile_specific_ext_hdr_id,
            ext_hdr_len,
            payload: vec![],
        }
    }
}

impl From<&[u8]> for RTPPacket {
    fn from(data: &[u8]) -> Self {
        let version = if data[0] & 0x80 != 0 { 2 } else { 1 };
        let padding = (data[0] & 0x20) > 0;
        let extension = (data[0] & 0x10) > 0;
        let cc = data[0] & 0xF;
        let marker = (data[1] & 0x80) > 0;
        let payload_type = data[1] & 0x7F;
        let seq_num = (data[2] as u16) << 8 | data[3] as u16;
        //TODO: abhi - create a struct to represent a raw packet and add utility methods to read
        //data like - read_unsigned_int(), read_byte(), etc.
        let timestamp = ((data[4] as u32) << 24)
            | ((data[5] as u32) << 16)
            | ((data[6] as u32) << 8)
            | data[7] as u32;

        let ssrc = ((data[8] as u32) << 24)
            | ((data[9] as u32) << 16)
            | ((data[10] as u32) << 8)
            | data[11] as u32;

        let mut i = 12usize;
        let mut csrcs = Vec::new();
        for _ in 0..cc {
            let csrc = ((data[i] as u32) << 24)
                | ((data[i + 1] as u32) << 16)
                | ((data[i + 2] as u32) << 8)
                | data[i + 3] as u32;
            csrcs.push(csrc);
            i += 4;
        }

        println!("{}", data[12]);
        RTPPacket {
            version,
            padding,
            extension,
            cc,
            marker,
            payload_type,
            seq_num,
            timestamp,
            ssrc,
            csrcs: if cc > 0 { Some(csrcs) } else { None },
            profile_specific_ext_hdr_id: None,
            ext_hdr_len: None,
            payload: data[i..].into(),
        }
    }
}
