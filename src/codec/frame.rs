use bytes::BytesMut;

#[derive(Debug)]
pub struct Message {
    pub id: i32,
    pub raw_id: String,
    pub len: i32,
    pub body: BytesMut,
}

#[derive(Debug)]
pub enum MessageFrame {
    Policy,
    Packet(Message),
}
