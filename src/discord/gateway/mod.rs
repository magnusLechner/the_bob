pub enum Opcode {
    DISPATCH,
    HEARTBEAT,
    IDENTIFY,
    STATUS_UPDATE,
    VOICE_STATUS_UPDATE,
    RESUME,
    RECONNECT,
    REQUEST_GUILD_MEMBERS,
    INVALID_SESSION,
    HELLO,
    HEARTBEAT_ACK
}

pub fn value_from_opcode (opcode: Opcode) -> u8 {
    match opcode {
        Opcode::DISPATCH => 0,
        Opcode::HEARTBEAT => 1,
        Opcode::IDENTIFY => 2,
        Opcode::STATUS_UPDATE => 3,
        Opcode::VOICE_STATUS_UPDATE => 4,
        Opcode::RESUME => 6,
        Opcode::RECONNECT => 7,
        Opcode::REQUEST_GUILD_MEMBERS => 8,
        Opcode::INVALID_SESSION => 9,
        Opcode::HELLO => 10,
        Opcode::HEARTBEAT_ACK => 11
    }
}

pub enum CloseEventCode {
    UNKNOWN_ERROR,
    UNKNOWN_OPCODE,
    DECODE_ERROR,
    NOT_AUTHENTICATED,
    AUTHENTICATION_FAILED,
    ALREADY_AUTHENTICATED,
    INVALID_SEQUENCE,
    RATE_LIMIT,
    SESSION_TIMEOUT,
    INVALID_SHARD,
    SHARDING_REQUIRED
}

pub fn value_from_close_event_code(event_code: CloseEventCode) -> i32 {
    match opcode {
        CloseEventCode::UNKNOWN_ERROR => 4000,
        CloseEventCode::UNKNOWN_OPCODE => 4001,
        CloseEventCode::DECODE_ERROR => 4002,
        CloseEventCode::NOT_AUTHENTICATED => 4003,
        CloseEventCode::AUTHENTICATION_FAILED => 4004,
        CloseEventCode::ALREADY_AUTHENTICATED => 4005,
        CloseEventCode::INVALID_SEQUENCE => 4007,
        CloseEventCode::RATE_LIMIT => 4008,
        CloseEventCode::SESSION_TIMEOUT => 4009,
        CloseEventCode::INVALID_SHARD => 4010,
        CloseEventCode::SHARDING_REQUIRED => 4011
    }
}

//TODO https://discordapp.com/developers/docs/topics/gateway#commands-and-events-gateway-events
pub struct Payload {
    op: u8,
    d: i32, //json
    s: i32,
    t: i32 //enum
}