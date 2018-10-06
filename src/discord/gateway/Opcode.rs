#[derive(Debug)]
pub enum OpcodeValue {
    Dispatch,
    Heartbeat,
    Identify,
    StatusUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatACK
}

pub fn get_opcode_value(opcode: OpcodeValue) -> u8 {
    match gateway_opcode {
        OpcodeValue::Dispatch => 0,
        OpcodeValue::Heartbeat => 1,
        OpcodeValue::Identify => 2,
        OpcodeValue::StatusUpdate => 3,
        OpcodeValue::VoiceStateUpdate => 4,
        OpcodeValue::Resume => 6,
        OpcodeValue::Reconnect => 7,
        OpcodeValue::RequestGuildMembers => 8,
        OpcodeValue::InvalidSession => 9,
        OpcodeValue::Hello => 10,
        OpcodeValue::HeartbeatACK => 11
    }
}