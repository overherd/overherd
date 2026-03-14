// LOCAL COMMANDS
pub const PUBLISH_CMD: &[u8; 4] = b"PUBL";
pub const NEIGHBOUR_LIST_CMD: &[u8; 4] = b"LSNB";
pub const NEIGHBOUR_ADD_CMD: &[u8; 4] = b"ADDN";

// REMOTE COMMANDS
pub const REQUEST_ID_CMD: &[u8; 4] = b"RQID";
pub const RESPONSE_ID_CMD: &[u8; 4] = b"RSID";
pub const BROADCAST_CMD: &[u8; 4] = b"BROD";
pub const REQUEST_PEERS_CMD: &[u8; 4] = b"RQPE";
pub const RESPONSE_PEERS_CMD: &[u8; 4] = b"RSPE";

// REPLIES
pub const OK_REPLY: &[u8; 3] = b"OK\n";
pub const NO_REPLY: &[u8; 3] = b"NO\n";
