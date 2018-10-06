#[derive(Debug)]
pub struct Payload {
    opcode: u8,
    event_data: String,
    sequence_number: i32,
    event_name: String
}

pub fn build_payload(opcode: u8, event_data: String, sequence_number: i32, event_name: String) -> Payload {
    Payload {
        opcode,
        event_data,
        sequence_number,
        event_name
    }
}