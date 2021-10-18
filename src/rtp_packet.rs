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

impl From<RTPPacket> for Vec<u8> {
    /*
     *
        0                   1                   2                   3
        0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
       |V=2|P|X|  CC   |M|     PT      |       sequence number         |
       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
       |                           timestamp                           |
       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
       |           synchronization source (SSRC) identifier            |
       +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
       |            contributing source (CSRC) identifiers             |
       |                             ....                              |
       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    */
    fn from(packet: RTPPacket) -> Self {
        // TODO abhi: fix the capacity
        let mut bytes = vec![0; 2];
        let header_byte: u8 = 0x80
            | if packet.padding { 1 << 5 } else { 0 }
            | if packet.extension { 1 << 4 } else { 0 }
            | packet.cc;
        bytes[0] = header_byte;
        bytes[1] = if packet.marker { 0x80 } else { 0 } | packet.payload_type;
        bytes.extend_from_slice(&packet.seq_num.to_be_bytes());
        bytes.extend_from_slice(&packet.timestamp.to_be_bytes());
        bytes.extend_from_slice(&packet.ssrc.to_be_bytes());
        if let Some(csrcs) = packet.csrcs {
            for quad in csrcs {
                bytes.extend_from_slice(&quad.to_be_bytes());
            }
        }
        if let Some(profile_specific_ext_hdr_id) = packet.profile_specific_ext_hdr_id {
            bytes.extend_from_slice(&profile_specific_ext_hdr_id.to_be_bytes());
        }
        if let Some(ext_hdr_len) = packet.ext_hdr_len {
            bytes.extend_from_slice(&ext_hdr_len.to_be_bytes());
        }
        bytes.extend(packet.payload);

        bytes
    }
}
