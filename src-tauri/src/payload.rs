use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub packet: Vec<u8>,
    pub port: usize
}