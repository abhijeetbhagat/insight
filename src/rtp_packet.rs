struct RTPPacket {
    version : u8,
    padding : bool,
    extension : bool,
    marker : bool,
    payload_type : u8,
    seq_num : u16,
    timestamp : u32,
    ssrc : u32
}
