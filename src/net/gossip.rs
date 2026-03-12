// vim: fdm=indent fdn=1
//
// Functions for managing the network communication protocol

use crate::net::{list::get_peer_list, remote};
use std::collections::HashSet;

const MAX_PEERS: usize = 4;

pub async fn refresh_peers() {
    let peers = get_peer_list().await.unwrap_or(HashSet::new());

    // TODO validate current peer list, drop unresponsive peers

    if peers.len() < MAX_PEERS {
        let mut new_peers: HashSet<String> = HashSet::new();
        for p in peers {
            let more_peers = remote::request_peers(p).await.unwrap_or(Vec::new());
            if !more_peers.is_empty() {
                println!("{}", more_peers.join(":"));
                for mp in more_peers {
                    new_peers.insert(mp);
                }
            }
        }
    }

    // TODO validate new peers
}
