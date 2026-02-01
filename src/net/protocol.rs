// LOCAL COMMANDS
pub const PUBLISH_CMD: &[u8; 4] = b"PUBL";
pub const NEIGHBOUR_LIST_CMD: &[u8; 4] = b"LSNB";
pub const NEIGHBOUR_ADD_CMD: &[u8; 4] = b"ADDN";

// PUBLIC COMMANDS
pub const BROADCAST_CMD: &[u8; 4] = b"BROD";
pub const REQ_PEERS_CMD: &[u8; 4] = b"REQP";
pub const RES_PEERS_CMD: &[u8; 4] = b"RESP";


// REPLIES
pub const OK_REPLY: &[u8;3] = b"OK\n";
pub const NO_REPLY: &[u8;3] = b"NO\n";
