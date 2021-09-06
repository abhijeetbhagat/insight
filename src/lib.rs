pub enum MediaType {
    Video,
    Audio,
    All,
}

pub mod connection;
pub mod response;
pub mod rtp_packet;
mod utils;
