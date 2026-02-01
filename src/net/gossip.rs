use crate::net::{list::get_list, remote};

const MAX_PEERS: usize = 4;

pub async fn refresh_peers() {
    let peers = get_list().await.unwrap_or(Vec::new());

    // TODO validate current peer list, drop unresponsive peers

    if peers.len() < MAX_PEERS {
        for p in peers {
            let more_peers = remote::request_peers(p).await.unwrap_or(Vec::new());
            if !more_peers.is_empty() {
                println!("{}", more_peers.join(":"));
            }
        }
    }
}
